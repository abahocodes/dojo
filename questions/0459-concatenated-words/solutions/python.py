def find_all_concatenated_words(words: list[str]) -> list[str]:
    order = sorted(range(len(words)), key=lambda i: len(words[i]))
    known = set()
    found = [False] * len(words)
    for i in order:
        w = words[i]
        if known:
            n = len(w)
            can = [False] * (n + 1)
            can[0] = True
            for end in range(1, n + 1):
                for start in range(end):
                    if can[start] and w[start:end] in known:
                        can[end] = True
                        break
            found[i] = can[n]
        known.add(w)
    return [w for i, w in enumerate(words) if found[i]]
