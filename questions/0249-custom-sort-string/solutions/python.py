def custom_sort_string(order: str, s: str) -> str:
    counts = {}
    for c in s:
        counts[c] = counts.get(c, 0) + 1
    ranked = set(order)
    head = "".join(c * counts.get(c, 0) for c in order)
    tail = "".join(c for c in s if c not in ranked)
    return head + tail
