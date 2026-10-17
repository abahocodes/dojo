class MinStack:
    def __init__(self):
        # Each entry is (value, minimum of this entry and everything below it).
        self.items: list[tuple[int, int]] = []

    def push(self, val: int) -> None:
        low = min(val, self.items[-1][1]) if self.items else val
        self.items.append((val, low))

    def pop(self) -> None:
        self.items.pop()

    def top(self) -> int:
        return self.items[-1][0]

    def get_min(self) -> int:
        return self.items[-1][1]


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
