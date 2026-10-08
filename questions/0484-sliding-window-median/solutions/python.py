import heapq
from collections import defaultdict


def median_sliding_window(nums: list[int], k: int) -> list[float]:
    low = []   # max-heap (values negated): the smaller half of the window
    high = []  # min-heap: the larger half of the window
    delayed = defaultdict(int)  # value -> copies removed but still inside a heap
    low_size = high_size = 0    # live elements in each heap

    def prune(heap, sign):
        # Drop removed values sitting on top of a heap.
        while heap and delayed[sign * heap[0]]:
            delayed[sign * heap[0]] -= 1
            heapq.heappop(heap)

    def rebalance():
        nonlocal low_size, high_size
        # Keep low_size == high_size or low_size == high_size + 1.
        if low_size > high_size + 1:
            heapq.heappush(high, -heapq.heappop(low))
            low_size -= 1
            high_size += 1
            prune(low, -1)
        elif low_size < high_size:
            heapq.heappush(low, -heapq.heappop(high))
            low_size += 1
            high_size -= 1
            prune(high, 1)

    def add(x):
        nonlocal low_size, high_size
        if not low or x <= -low[0]:
            heapq.heappush(low, -x)
            low_size += 1
        else:
            heapq.heappush(high, x)
            high_size += 1
        rebalance()

    def remove(x):
        nonlocal low_size, high_size
        delayed[x] += 1
        if x <= -low[0]:
            low_size -= 1
            if x == -low[0]:
                prune(low, -1)
        else:
            high_size -= 1
            if high and x == high[0]:
                prune(high, 1)
        rebalance()

    def median():
        if k % 2 == 1:
            return float(-low[0])
        return (-low[0] + high[0]) / 2

    result = []
    for i, x in enumerate(nums):
        add(x)
        if i >= k:
            remove(nums[i - k])
        if i >= k - 1:
            result.append(median())
    return result
