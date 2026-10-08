def close_strings(word1: str, word2: str) -> bool:
    if len(word1) != len(word2):
        return False
    a = [0] * 26
    b = [0] * 26
    for ch in word1:
        a[ord(ch) - 97] += 1
    for ch in word2:
        b[ord(ch) - 97] += 1
    for x, y in zip(a, b):
        if (x == 0) != (y == 0):
            return False
    return sorted(a) == sorted(b)
