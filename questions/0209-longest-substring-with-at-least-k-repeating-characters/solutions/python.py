def longest_substring_k_repeating(s: str, k: int) -> int:
    best = 0
    for limit in range(1, 27):
        count = [0] * 26
        left = 0
        unique = 0
        at_least = 0
        for right, ch in enumerate(s):
            c = ord(ch) - 97
            if count[c] == 0:
                unique += 1
            count[c] += 1
            if count[c] == k:
                at_least += 1
            while unique > limit:
                d = ord(s[left]) - 97
                if count[d] == k:
                    at_least -= 1
                count[d] -= 1
                if count[d] == 0:
                    unique -= 1
                left += 1
            if unique == at_least:
                best = max(best, right - left + 1)
    return best
