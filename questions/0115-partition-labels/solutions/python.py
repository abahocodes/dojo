def partition_labels(s: str) -> list[int]:
    last = {ch: i for i, ch in enumerate(s)}
    sizes = []
    start = end = 0
    for i, ch in enumerate(s):
        end = max(end, last[ch])
        if i == end:
            sizes.append(i - start + 1)
            start = i + 1
    return sizes
