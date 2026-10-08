import heapq


def most_booked(n: int, meetings: list[list[int]]) -> int:
    free = list(range(n))  # min-heap of free room numbers
    busy = []              # min-heap of (end time, room)
    count = [0] * n
    for start, end in sorted(meetings):
        while busy and busy[0][0] <= start:
            _, room = heapq.heappop(busy)
            heapq.heappush(free, room)
        if free:
            room = heapq.heappop(free)
            heapq.heappush(busy, (end, room))
        else:
            # Wait for the earliest room; ties go to the lower number.
            free_at, room = heapq.heappop(busy)
            heapq.heappush(busy, (free_at + end - start, room))
        count[room] += 1
    return count.index(max(count))
