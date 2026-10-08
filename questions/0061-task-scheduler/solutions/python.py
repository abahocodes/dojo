from collections import Counter

def least_interval(tasks: list[str], n: int) -> int:
    counts = Counter(tasks).values()
    most = max(counts)
    tied = sum(1 for c in counts if c == most)
    # most - 1 full rows of width n + 1, then one slot per label tied for the top count
    return max(len(tasks), (most - 1) * (n + 1) + tied)
