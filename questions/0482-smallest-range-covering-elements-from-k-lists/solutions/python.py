import heapq


def smallest_range(nums: list[list[int]]) -> list[int]:
    heap = [(lst[0], r, 0) for r, lst in enumerate(nums)]  # (value, list, position)
    heapq.heapify(heap)
    high = max(lst[0] for lst in nums)
    best = [heap[0][0], high]
    while True:
        low, r, c = heapq.heappop(heap)
        if high - low < best[1] - best[0]:
            best = [low, high]
        if c + 1 == len(nums[r]):
            return best
        nxt = nums[r][c + 1]
        high = max(high, nxt)
        heapq.heappush(heap, (nxt, r, c + 1))
