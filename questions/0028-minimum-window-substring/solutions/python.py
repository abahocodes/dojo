from collections import Counter


def min_window(s: str, t: str) -> str:
    need = Counter(t)
    missing = len(t)
    best_start, best_len = 0, len(s) + 1
    left = 0
    for right, ch in enumerate(s):
        if need[ch] > 0:
            missing -= 1
        need[ch] -= 1
        while missing == 0:
            if right - left + 1 < best_len:
                best_start, best_len = left, right - left + 1
            out = s[left]
            need[out] += 1
            if need[out] > 0:
                missing += 1
            left += 1
    if best_len > len(s):
        return ""
    return s[best_start:best_start + best_len]
