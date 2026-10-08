def wonderful_substrings(word: str) -> int:
    seen = [0] * 1024
    seen[0] = 1
    mask = 0
    total = 0
    for ch in word:
        mask ^= 1 << (ord(ch) - ord("a"))
        total += seen[mask]
        for k in range(10):
            total += seen[mask ^ (1 << k)]
        seen[mask] += 1
    return total
