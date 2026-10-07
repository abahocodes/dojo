def length_of_longest_substring(s: str) -> int:
    last = {}
    left = 0
    best = 0
    for i, ch in enumerate(s):
        if ch in last and last[ch] >= left:
            left = last[ch] + 1
        last[ch] = i
        best = max(best, i - left + 1)
    return best
