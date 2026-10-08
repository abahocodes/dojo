import heapq


def k_closest(points: list[list[int]], k: int) -> list[list[int]]:
    # Max-heap (negated squared distance) holding the k closest points so far.
    heap = []
    for i, (x, y) in enumerate(points):
        d = x * x + y * y
        if len(heap) < k:
            heapq.heappush(heap, (-d, i))
        elif d < -heap[0][0]:
            heapq.heapreplace(heap, (-d, i))
    return [points[i] for _, i in heap]
