# dojo python harness. Usage: python3 harness.py <solution.py> <spec.json> <results.json>
#
# spec.json:    {"function", "params": [{"name","type"}], "returns", "timeout_secs",
#                "cases": [{"index", "input": {...}}]}
# results.json: {"fatal": str} or {"results": [{"index","status","got","error","stdout","ms"}]},
#               plus "load_stdout": what the file printed while being loaded
# status is one of: ok, error, timeout. Comparison happens in dojo, not here.

import contextlib
import importlib.util
import io
import json
import os
import signal
import sys
import time
import traceback

STDOUT_CAP = 4000
MAX_NODES = 100_000


class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

    def __repr__(self):
        return f"ListNode({self.val})"


class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self):
        return f"TreeNode({self.val})"


class CaseTimeout(BaseException):
    """BaseException so a bare `except Exception` in user code can't swallow it."""


def to_list_node(values):
    dummy = ListNode()
    tail = dummy
    for v in values:
        tail.next = ListNode(v)
        tail = tail.next
    return dummy.next


def from_list_node(node):
    out = []
    while node is not None:
        if len(out) >= MAX_NODES:
            raise ValueError("returned linked list is too long (cycle?)")
        out.append(node.val)
        node = node.next
    return out


def to_tree(values):
    if not values or values[0] is None:
        return None
    root = TreeNode(values[0])
    queue = [root]
    i = 1
    for node in queue:
        if i >= len(values):
            break
        if values[i] is not None:
            node.left = TreeNode(values[i])
            queue.append(node.left)
        i += 1
        if i < len(values) and values[i] is not None:
            node.right = TreeNode(values[i])
            queue.append(node.right)
        i += 1
    return root


def from_tree(root):
    out = []
    queue = [root]
    for node in queue:
        if len(out) >= MAX_NODES:
            raise ValueError("returned tree is too large (cycle?)")
        if node is None:
            out.append(None)
            continue
        out.append(node.val)
        queue.append(node.left)
        queue.append(node.right)
    while out and out[-1] is None:
        out.pop()
    return out


def decode(ty, v):
    if ty.endswith("[]"):
        return None if v is None else [decode(ty[:-2], x) for x in v]
    if ty == "ListNode":
        return to_list_node(v)
    if ty == "TreeNode":
        return to_tree(v)
    return v


def normalize(v):
    if isinstance(v, (list, tuple)):
        return [normalize(x) for x in v]
    if isinstance(v, dict):
        return {str(k): normalize(x) for k, x in v.items()}
    return v


def encode(ty, v):
    if ty.endswith("[]"):
        return None if v is None else [encode(ty[:-2], x) for x in v]
    if ty == "ListNode":
        return from_list_node(v)
    if ty == "TreeNode":
        return from_tree(v)
    return normalize(v)


def format_error(exc, solution_path):
    frames = [f for f in traceback.extract_tb(exc.__traceback__) if f.filename == solution_path]
    out = ""
    if frames:
        out += "Traceback (most recent call last):\n" + "".join(traceback.format_list(frames))
    out += "".join(traceback.format_exception_only(type(exc), exc))
    # Show `solution.py`, not the full workspace path.
    return out.replace(solution_path, os.path.basename(solution_path)).rstrip()


def on_alarm(signum, frame):
    raise CaseTimeout()


def main():
    solution_path, spec_path, results_path = sys.argv[1:4]
    with open(spec_path) as f:
        spec = json.load(f)

    def write(obj):
        with open(results_path, "w") as f:
            json.dump(obj, f)

    sys.setrecursionlimit(20_000)
    module_spec = importlib.util.spec_from_file_location("solution", solution_path)
    module = importlib.util.module_from_spec(module_spec)
    module.ListNode = ListNode
    module.TreeNode = TreeNode
    # Prints at the top level of the file are shown too ("printed while loading").
    loading = io.StringIO()
    try:
        with contextlib.redirect_stdout(loading):
            module_spec.loader.exec_module(module)
    except BaseException as e:
        write({"fatal": format_error(e, solution_path), "load_stdout": loading.getvalue()[:STDOUT_CAP]})
        return

    fn = getattr(module, spec["function"], None)
    if not callable(fn):
        write({"fatal": f"function `{spec['function']}` not found in {solution_path}"})
        return

    signal.signal(signal.SIGALRM, on_alarm)
    results = []
    for case in spec["cases"]:
        result = {"index": case["index"]}
        buf = io.StringIO()
        start = time.perf_counter()
        try:
            args = [decode(p["type"], case["input"][p["name"]]) for p in spec["params"]]
            with contextlib.redirect_stdout(buf):
                signal.setitimer(signal.ITIMER_REAL, spec["timeout_secs"])
                try:
                    got = fn(*args)
                finally:
                    signal.setitimer(signal.ITIMER_REAL, 0)
            got = encode(spec["returns"], got)
            json.dumps(got)  # must be serializable
            result["status"] = "ok"
            result["got"] = got
        except CaseTimeout:
            result["status"] = "timeout"
        except BaseException as e:
            result["status"] = "error"
            result["error"] = format_error(e, solution_path)
        result["ms"] = round((time.perf_counter() - start) * 1000, 2)
        printed = buf.getvalue()
        if printed:
            result["stdout"] = printed[:STDOUT_CAP]
        results.append(result)

    write({"results": results, "load_stdout": loading.getvalue()[:STDOUT_CAP]})


if __name__ == "__main__":
    main()
