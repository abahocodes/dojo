import heapq


def max_score(nums1: list[int], nums2: list[int], k: int) -> int:
    pairs = sorted(zip(nums2, nums1), reverse=True)
    chosen = []  # min-heap of the k largest nums1 values seen so far
    total = 0
    best = 0
    for m, x in pairs:
        heapq.heappush(chosen, x)
        total += x
        if len(chosen) > k:
            total -= heapq.heappop(chosen)
        if len(chosen) == k:
            best = max(best, total * m)
    return best
