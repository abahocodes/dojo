def remove_outer_parentheses(s: str) -> str:
    out = []
    depth = 0
    for ch in s:
        if ch == "(":
            if depth > 0:
                out.append(ch)
            depth += 1
        else:
            depth -= 1
            if depth > 0:
                out.append(ch)
    return "".join(out)
