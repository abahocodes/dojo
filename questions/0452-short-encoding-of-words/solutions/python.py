def minimum_length_encoding(words: list[str]) -> int:
    keep = set(words)
    for w in set(words):
        for k in range(1, len(w)):
            keep.discard(w[k:])
    return sum(len(w) + 1 for w in keep)
