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


class CappedOutput(io.TextIOBase):
    """Collects printed text up to STDOUT_CAP; the rest is dropped as it's
    printed, so a print inside an infinite loop can't exhaust memory."""

    def __init__(self):
        self.parts = []
        self.size = 0
        self.dropped = False

    def writable(self):
        return True

    def write(self, s):
        if self.size < STDOUT_CAP:
            room = STDOUT_CAP - self.size
            self.parts.append(s[:room])
            self.size += min(len(s), room)
            if len(s) > room:
                self.dropped = True
        else:
            self.dropped = True
        return len(s)

    def text(self):
        out = "".join(self.parts)
        if self.dropped:
            out += "\n… (more output not shown)"
        return out


def explain(exc, solution_path):
    """The error, plus a hint for the usual runaway cases."""
    text = format_error(exc, solution_path)
    if isinstance(exc, RecursionError):
        text += (
            f"\nhint: recursion went deeper than {sys.getrecursionlimit():,} calls: "
            "a missing base case, or recursion too deep for this input (try an explicit stack)"
        )
    elif isinstance(exc, MemoryError):
        text += "\nhint: ran out of memory: a structure growing without bound?"
    return text


def main():
    solution_path, spec_path, results_path = sys.argv[1:4]
    progress_path = results_path + ".progress"
    with open(spec_path) as f:
        spec = json.load(f)

    def write(obj):
        with open(results_path, "w") as f:
            json.dump(obj, f)

    def progress(step):
        # dojo watches this file and stops a step that stalls (an
        # uninterruptible loop in C code, a crash) from the outside.
        with open(progress_path, "w") as f:
            f.write(str(step))

    if sys.platform.startswith("linux"):
        try:
            import resource

            limit = 2 * 1024**3
            resource.setrlimit(resource.RLIMIT_AS, (limit, limit))
        except (ImportError, ValueError, OSError):
            pass

    sys.setrecursionlimit(20_000)
    module_spec = importlib.util.spec_from_file_location("solution", solution_path)
    module = importlib.util.module_from_spec(module_spec)
    module.ListNode = ListNode
    module.TreeNode = TreeNode
    signal.signal(signal.SIGALRM, on_alarm)

    # Prints at the top level of the file are shown too ("printed while loading").
    loading = CappedOutput()
    progress("load")
    try:
        with contextlib.redirect_stdout(loading):
            signal.setitimer(signal.ITIMER_REAL, spec["timeout_secs"])
            try:
                module_spec.loader.exec_module(module)
            finally:
                signal.setitimer(signal.ITIMER_REAL, 0)
    except CaseTimeout:
        write({
            "fatal": f"loading {os.path.basename(solution_path)} took over {spec['timeout_secs']:g}s: "
            "is there a loop at the top level of the file?",
            "load_stdout": loading.text(),
        })
        return
    except BaseException as e:
        write({"fatal": explain(e, solution_path), "load_stdout": loading.text()})
        return

    fn = getattr(module, spec["function"], None)
    if not callable(fn):
        write({"fatal": f"function `{spec['function']}` not found in {solution_path}"})
        return

    results = []
    stopped = False
    for case in spec["cases"]:
        result = {"index": case["index"]}
        if stopped:
            # After a timeout the rest would most likely time out too.
            result["status"] = "not_run"
            results.append(result)
            continue
        progress(case["index"])
        buf = CappedOutput()
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
            stopped = True
        except BaseException as e:
            result["status"] = "error"
            result["error"] = explain(e, solution_path)
        result["ms"] = round((time.perf_counter() - start) * 1000, 2)
        printed = buf.text()
        if printed:
            result["stdout"] = printed
        results.append(result)
        # Partial results survive a crash on a later case.
        write({"results": results, "load_stdout": loading.text()})

    progress("done")
    write({"results": results, "load_stdout": loading.text()})


if __name__ == "__main__":
    main()
