# Hints

## Hint 1
Sort the meetings by start time and simulate. For every meeting you need two
answers fast: "which is the lowest-numbered free room?" and, if none, "which
room frees up first?"

## Hint 2
Use two min-heaps: one of free room numbers, and one of busy rooms keyed by
`(time it frees up, room number)`. Before placing a meeting, move every busy
room whose end time is at most the meeting's start back into the free heap.

## Hint 3
If the free heap is empty, pop the busy room with the smallest
`(end, room)` pair and push it back with end `end + duration` of the delayed
meeting. Count meetings per room, and pick the lowest index with the maximum
count. Delayed end times can exceed `2^31 - 1`, so store them in 64 bits.
