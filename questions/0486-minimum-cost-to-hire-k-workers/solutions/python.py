import heapq
from functools import cmp_to_key


def mincost_to_hire_workers(quality: list[int], wage: list[int], k: int) -> float:
    # Sort by wage/quality ratio, compared exactly with cross-multiplication.
    workers = sorted(
        zip(quality, wage),
        key=cmp_to_key(lambda a, b: a[1] * b[0] - b[1] * a[0]),
    )
    heap = []  # max-heap (negated) of the qualities in the current group
    total_quality = 0
    best = float("inf")
    for q, w in workers:
        heapq.heappush(heap, -q)
        total_quality += q
        if len(heap) > k:
            total_quality += heapq.heappop(heap)  # drop the largest quality
        if len(heap) == k:
            # This worker has the largest ratio so far and sets the pay rate.
            best = min(best, total_quality * w / q)
    return best
