import heapq


def last_stone_weight(stones: list[int]) -> int:
    # heapq is a min-heap, so store negated weights to pop the heaviest first.
    heap = [-s for s in stones]
    heapq.heapify(heap)
    while len(heap) > 1:
        y = -heapq.heappop(heap)
        x = -heapq.heappop(heap)
        if y != x:
            heapq.heappush(heap, -(y - x))
    return -heap[0] if heap else 0
