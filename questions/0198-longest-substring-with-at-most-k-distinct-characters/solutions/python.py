def length_of_longest_substring_k_distinct(s: str, k: int) -> int:
    count = [0] * 128
    distinct = 0
    left = 0
    best = 0
    for right, ch in enumerate(s):
        c = ord(ch)
        if count[c] == 0:
            distinct += 1
        count[c] += 1
        while distinct > k:
            d = ord(s[left])
            count[d] -= 1
            if count[d] == 0:
                distinct -= 1
            left += 1
        best = max(best, right - left + 1)
    return best
