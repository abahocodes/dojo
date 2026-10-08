def min_remove_to_make_valid(s: str) -> str:
    chars = list(s)
    stack = []
    for i, c in enumerate(chars):
        if c == "(":
            stack.append(i)
        elif c == ")":
            if stack:
                stack.pop()
            else:
                chars[i] = ""
    for i in stack:
        chars[i] = ""
    return "".join(chars)
