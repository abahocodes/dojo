def remove_kdigits(num: str, k: int) -> str:
    stack = []
    for d in num:
        while k > 0 and stack and stack[-1] > d:
            stack.pop()
            k -= 1
        stack.append(d)
    if k > 0:
        del stack[-k:]
    result = "".join(stack).lstrip("0")
    return result or "0"
