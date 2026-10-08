def num_decodings(s: str) -> int:
    prev, curr = 1, 1 if s[0] != "0" else 0
    for i in range(2, len(s) + 1):
        nxt = 0
        if s[i - 1] != "0":
            nxt += curr
        if 10 <= int(s[i - 2:i]) <= 26:
            nxt += prev
        prev, curr = curr, nxt
    return curr
