def longest_awesome(s: str) -> int:
    # first[m] = earliest prefix index whose digit-parity mask is m
    n = len(s)
    first = [n + 1] * 1024
    first[0] = 0
    mask = 0
    best = 0
    for i, ch in enumerate(s, 1):
        mask ^= 1 << (ord(ch) - 48)
        # all digit counts even
        best = max(best, i - first[mask])
        # exactly one digit count odd
        for d in range(10):
            best = max(best, i - first[mask ^ (1 << d)])
        if first[mask] > i:
            first[mask] = i
    return best
