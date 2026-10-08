from collections import Counter


def find_substring(s: str, words: list[str]) -> list[int]:
    L, m = len(words[0]), len(words)
    if m * L > len(s):
        return []
    need = Counter(words)
    result = []
    for r in range(L):
        have = Counter()
        left = r
        count = 0
        for right in range(r, len(s) - L + 1, L):
            w = s[right:right + L]
            if w not in need:
                have.clear()
                count = 0
                left = right + L
                continue
            have[w] += 1
            count += 1
            while have[w] > need[w]:
                have[s[left:left + L]] -= 1
                count -= 1
                left += L
            if count == m:
                result.append(left)
                have[s[left:left + L]] -= 1
                count -= 1
                left += L
    result.sort()
    return result
