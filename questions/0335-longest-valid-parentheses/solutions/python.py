def longest_valid_parentheses(s: str) -> int:
    stack = [-1]  # bottom entry is the last unmatched position
    best = 0
    for i, ch in enumerate(s):
        if ch == "(":
            stack.append(i)
        else:
            stack.pop()
            if not stack:
                stack.append(i)
            else:
                best = max(best, i - stack[-1])
    return best
