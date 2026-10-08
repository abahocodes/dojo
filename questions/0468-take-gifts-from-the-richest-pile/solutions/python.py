import heapq
import math


def pick_gifts(gifts: list[int], k: int) -> int:
    # heapq is a min-heap, so store negated sizes to pop the largest pile.
    heap = [-g for g in gifts]
    heapq.heapify(heap)
    for _ in range(k):
        largest = -heap[0]
        heapq.heapreplace(heap, -math.isqrt(largest))
    return -sum(heap)
