def find_the_longest_substring(s: str) -> int:
    bit = {"a": 1, "e": 2, "i": 4, "o": 8, "u": 16}
    first = [None] * 32
    first[0] = -1
    mask = 0
    best = 0
    for i, ch in enumerate(s):
        mask ^= bit.get(ch, 0)
        if first[mask] is None:
            first[mask] = i
        else:
            best = max(best, i - first[mask])
    return best
