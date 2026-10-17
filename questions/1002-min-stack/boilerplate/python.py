# 1002. Min Stack (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class MinStack:
    def __init__(self):
        pass

    def push(self, val: int) -> None:
        pass

    def pop(self) -> None:
        pass

    def top(self) -> int:
        pass

    def get_min(self) -> int:
        pass


# dojo calls this to run the tests; you don't need to change it.
def min_stack(ops: list[str], vals: list[int]) -> list[int]:
    stack = MinStack()
    out = []
    for op, val in zip(ops, vals):
        if op == "push":
            stack.push(val)
        elif op == "pop":
            stack.pop()
        elif op == "top":
            out.append(stack.top())
        else:
            out.append(stack.get_min())
    return out
