from collections import Counter


def unique_occurrences(arr: list[int]) -> bool:
    freqs = Counter(arr).values()
    return len(set(freqs)) == len(freqs)
