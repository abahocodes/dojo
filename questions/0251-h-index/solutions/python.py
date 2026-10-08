def h_index(citations: list[int]) -> int:
    n = len(citations)
    buckets = [0] * (n + 1)  # buckets[n] holds every paper with >= n citations
    for c in citations:
        buckets[min(c, n)] += 1
    papers = 0
    for h in range(n, -1, -1):
        papers += buckets[h]
        if papers >= h:
            return h
    return 0
