def shifting_letters(s: str, shifts: list[list[int]]) -> str:
    n = len(s)
    diff = [0] * (n + 1)
    for start, end, direction in shifts:
        delta = 1 if direction == 1 else -1
        diff[start] += delta
        diff[end + 1] -= delta
    out = []
    net = 0
    for i, ch in enumerate(s):
        net += diff[i]
        out.append(chr((ord(ch) - ord("a") + net) % 26 + ord("a")))
    return "".join(out)
