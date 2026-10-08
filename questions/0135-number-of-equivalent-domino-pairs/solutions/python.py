def num_equiv_domino_pairs(dominoes: list[list[int]]) -> int:
    seen = [0] * 100
    pairs = 0
    for a, b in dominoes:
        key = 10 * min(a, b) + max(a, b)
        pairs += seen[key]
        seen[key] += 1
    return pairs
