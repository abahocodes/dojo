def character_replacement(s: str, k: int) -> int:
    counts = [0] * 26
    max_freq = 0
    left = 0
    for right, ch in enumerate(s):
        idx = ord(ch) - 65
        counts[idx] += 1
        if counts[idx] > max_freq:
            max_freq = counts[idx]
        if right - left + 1 - max_freq > k:
            counts[ord(s[left]) - 65] -= 1
            left += 1
    return len(s) - left
