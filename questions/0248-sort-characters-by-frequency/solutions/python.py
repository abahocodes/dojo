from collections import Counter


def frequency_sort(s: str) -> str:
    counts = Counter(s)
    order = sorted(counts, key=lambda c: (-counts[c], c))
    return "".join(c * counts[c] for c in order)
