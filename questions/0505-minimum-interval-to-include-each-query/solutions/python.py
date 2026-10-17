import heapq


def min_interval(intervals: list[list[int]], queries: list[int]) -> list[int]:
    intervals = sorted(intervals)
    order = sorted(range(len(queries)), key=lambda i: queries[i])
    answer = [-1] * len(queries)
    heap = []  # (length, right end) of intervals that start at or before q
    i = 0
    for qi in order:
        q = queries[qi]
        while i < len(intervals) and intervals[i][0] <= q:
            left, right = intervals[i]
            heapq.heappush(heap, (right - left + 1, right))
            i += 1
        # Intervals that end before q can never contain a later (larger) query.
        while heap and heap[0][1] < q:
            heapq.heappop(heap)
        if heap:
            answer[qi] = heap[0][0]
    return answer
