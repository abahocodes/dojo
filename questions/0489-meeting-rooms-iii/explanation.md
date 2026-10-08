# Approach: simulation with two heaps

Process meetings in order of start time and keep two min-heaps:

- `free`: room numbers that are currently empty;
- `busy`: pairs `(end time, room)` for occupied rooms.

For a meeting `[start, end)`:

1. Release every busy room whose end time is `<= start` into `free`. Room
   numbers in `free` are interchangeable except for their number, so the heap
   gives the lowest one directly.
2. If `free` is not empty, pop the lowest room, and record it as busy until
   `end`.
3. Otherwise every room is busy. Pop the smallest `(end time, room)` pair,
   which is the room that frees up first, lowest number on ties. The meeting
   starts at that room's end time and keeps its duration, so push the room
   back with end time `end_time + (end - start)`.

Count the meetings each room hosts, and return the lowest room with the
maximum count.

```python
import heapq

def most_booked(n, meetings):
    free = list(range(n))
    busy = []
    count = [0] * n
    for start, end in sorted(meetings):
        while busy and busy[0][0] <= start:
            _, room = heapq.heappop(busy)
            heapq.heappush(free, room)
        if free:
            room = heapq.heappop(free)
            heapq.heappush(busy, (end, room))
        else:
            free_at, room = heapq.heappop(busy)
            heapq.heappush(busy, (free_at + end - start, room))
        count[room] += 1
    return count.index(max(count))
```

## Complexity

- Time: O(m log m + m log n) for `m` meetings: the sort plus heap operations
  on heaps of at most `n` rooms.
- Space: O(n) for the heaps and counters (plus the sorted copy).

## Pitfalls

- Keeping only the busy heap and taking its top when rooms are free. Among
  free rooms the lowest number wins, not the one that freed earliest; that is
  why released rooms go into a separate heap.
- Treating a room that frees at exactly `start` as busy. Intervals are
  half-open.
- Breaking ties in `busy` by end time only. When several rooms free at the
  same moment, the lowest-numbered one must be chosen.
- 32-bit end times. Delays accumulate: with one room, `10^5` meetings each
  lasting nearly `5 * 10^5` push end times far past `2^31 - 1`.
- Processing meetings in input order. Assignment follows start times.
