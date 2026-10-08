def decode_string(s: str) -> str:
    stack = []
    buf = []
    k = 0
    for ch in s:
        if ch.isdigit():
            k = k * 10 + int(ch)
        elif ch == "[":
            stack.append((buf, k))
            buf, k = [], 0
        elif ch == "]":
            prev, times = stack.pop()
            prev.append("".join(buf) * times)
            buf = prev
        else:
            buf.append(ch)
    return "".join(buf)
