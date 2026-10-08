from collections import Counter


def top_k_frequent_words(words: list[str], k: int) -> list[str]:
    counts = Counter(words)
    ranked = sorted(counts, key=lambda w: (-counts[w], w))
    return ranked[:k]
