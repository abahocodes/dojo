def check_inclusion(s1: str, s2: str) -> bool:
    m = len(s1)
    if m > len(s2):
        return False
    need = [0] * 26
    have = [0] * 26
    for c in s1:
        need[ord(c) - 97] += 1
    matches = sum(1 for x in need if x == 0)

    for j in range(len(s2)):
        i = ord(s2[j]) - 97
        if have[i] == need[i]:
            matches -= 1
        have[i] += 1
        if have[i] == need[i]:
            matches += 1
        if j >= m:
            i = ord(s2[j - m]) - 97
            if have[i] == need[i]:
                matches -= 1
            have[i] -= 1
            if have[i] == need[i]:
                matches += 1
        if j >= m - 1 and matches == 26:
            return True
    return False
