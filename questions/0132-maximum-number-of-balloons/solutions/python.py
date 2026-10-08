from collections import Counter


def max_number_of_balloons(text: str) -> int:
    have = Counter(text)
    need = Counter("balloon")
    return min(have[c] // k for c, k in need.items())
