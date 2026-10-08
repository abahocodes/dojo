from collections import Counter


def uncommon_from_sentences(s1: str, s2: str) -> list[str]:
    counts = Counter(s1.split() + s2.split())
    return [w for w, c in counts.items() if c == 1]
