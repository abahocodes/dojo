import heapq


def kth_largest_stream(k: int, nums: list[int], adds: list[int]) -> list[int]:
    # Min-heap holding the k largest values seen so far; its top is the answer.
    heap: list[int] = []

    def add(value: int) -> None:
        if len(heap) < k:
            heapq.heappush(heap, value)
        elif value > heap[0]:
            heapq.heapreplace(heap, value)

    for v in nums:
        add(v)
    result = []
    for v in adds:
        add(v)
        result.append(heap[0])
    return result
