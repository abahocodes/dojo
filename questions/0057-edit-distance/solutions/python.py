def edit_distance(word1: str, word2: str) -> int:
    m = len(word2)
    prev = list(range(m + 1))
    for i, c1 in enumerate(word1, 1):
        curr = [i] + [0] * m
        for j, c2 in enumerate(word2, 1):
            if c1 == c2:
                curr[j] = prev[j - 1]
            else:
                curr[j] = 1 + min(prev[j], curr[j - 1], prev[j - 1])
        prev = curr
    return prev[m]
