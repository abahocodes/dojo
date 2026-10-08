import heapq

def find_kth_largest(nums: list[int], k: int) -> int:
    # min-heap holding the k largest values seen so far; heap[0] is the smallest of them
    heap = nums[:k]
    heapq.heapify(heap)
    for x in nums[k:]:
        if x > heap[0]:
            heapq.heapreplace(heap, x)
    return heap[0]
