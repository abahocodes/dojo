def group_anagrams(words: list[str]) -> list[list[str]]:
    groups = {}
    for w in words:
        counts = [0] * 26
        for ch in w:
            counts[ord(ch) - 97] += 1
        groups.setdefault(tuple(counts), []).append(w)
    return list(groups.values())
