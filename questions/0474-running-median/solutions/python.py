import heapq


def running_median(nums: list[int]) -> list[float]:
    low = []   # max-heap (negated) holding the smaller half
    high = []  # min-heap holding the larger half
    medians = []
    for x in nums:
        if not low or x <= -low[0]:
            heapq.heappush(low, -x)
        else:
            heapq.heappush(high, x)
        # Keep len(low) == len(high) or len(low) == len(high) + 1.
        if len(low) > len(high) + 1:
            heapq.heappush(high, -heapq.heappop(low))
        elif len(high) > len(low):
            heapq.heappush(low, -heapq.heappop(high))
        if len(low) > len(high):
            medians.append(float(-low[0]))
        else:
            medians.append((-low[0] + high[0]) / 2)
    return medians
