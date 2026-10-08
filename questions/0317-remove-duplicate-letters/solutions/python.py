def remove_duplicate_letters(s: str) -> str:
    last = {c: i for i, c in enumerate(s)}
    stack = []
    used = set()
    for i, c in enumerate(s):
        if c in used:
            continue
        while stack and stack[-1] > c and last[stack[-1]] > i:
            used.discard(stack.pop())
        stack.append(c)
        used.add(c)
    return "".join(stack)
