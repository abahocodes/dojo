def find_anagrams(s: str, p: str) -> list[int]:
    m = len(p)
    result = []
    if m > len(s):
        return result
    need = [0] * 26
    have = [0] * 26
    for c in p:
        need[ord(c) - 97] += 1
    matches = sum(1 for x in need if x == 0)

    for j in range(len(s)):
        i = ord(s[j]) - 97
        if have[i] == need[i]:
            matches -= 1
        have[i] += 1
        if have[i] == need[i]:
            matches += 1
        if j >= m:
            i = ord(s[j - m]) - 97
            if have[i] == need[i]:
                matches -= 1
            have[i] -= 1
            if have[i] == need[i]:
                matches += 1
        if j >= m - 1 and matches == 26:
            result.append(j - m + 1)
    return result
