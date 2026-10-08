import heapq


def merge_k_sorted_arrays(arrays: list[list[int]]) -> list[int]:
    # Heap of (value, array index, position) for the front of each array.
    heap = [(arr[0], a, 0) for a, arr in enumerate(arrays) if arr]
    heapq.heapify(heap)
    merged = []
    while heap:
        value, a, p = heap[0]
        merged.append(value)
        if p + 1 < len(arrays[a]):
            heapq.heapreplace(heap, (arrays[a][p + 1], a, p + 1))
        else:
            heapq.heappop(heap)
    return merged
