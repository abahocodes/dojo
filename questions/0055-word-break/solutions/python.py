def word_break(s: str, words: list[str]) -> bool:
    vocab = set(words)
    lengths = sorted({len(w) for w in words})
    ok = [True] + [False] * len(s)
    for i in range(1, len(s) + 1):
        for length in lengths:
            if length > i:
                break
            if ok[i - length] and s[i - length:i] in vocab:
                ok[i] = True
                break
    return ok[len(s)]
