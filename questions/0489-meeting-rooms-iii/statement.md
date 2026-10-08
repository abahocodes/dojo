An office has `n` meeting rooms numbered `0` to `n - 1`, all empty at the
start. You are given a list of meetings, where `meetings[i] = [start, end]`
describes a meeting that wants to run over the half-open interval
`[start, end)`. No two meetings share the same `start` time. The list is not
necessarily sorted.

Meetings are assigned one at a time, in order of their **original start
time**:

1. If any room is free at the meeting's start time, the meeting takes the
   free room with the **lowest number**. A room whose previous meeting ends
   exactly at this start time counts as free.
2. Otherwise the meeting waits for the room that becomes free **earliest**
   (the lowest-numbered one if several free up at the same moment). It starts
   the moment that room frees up and keeps its original length, so a meeting
   `[start, end]` delayed until time `t` occupies `[t, t + end - start)`.

Return the number of the room that hosted the most meetings. If several rooms
are tied, return the lowest-numbered one.

## Example 1

```
n        = 2
meetings = [[0, 10], [1, 5], [2, 7], [3, 4]]
output   = 0
```

- `[0,10)` takes room 0, `[1,5)` takes room 1.
- The meeting starting at 2 waits; room 1 frees at 5, so it runs `[5,10)` in
  room 1.
- The meeting starting at 3 waits; both rooms free at 10, so it takes room 0
  and runs `[10,11)`.

Each room hosted two meetings; the lower number, 0, wins the tie.

## Example 2

```
n        = 3
meetings = [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
output   = 1
```

Rooms 0, 1 and 2 take the first three meetings. The meeting starting at 4
waits for room 2 (free at 5) and runs `[5,10)`. The meeting starting at 6 waits
for the earliest free room: rooms 1 and 2 both free at 10, so it goes to
room 1. Room 1 and room 2 each hosted two meetings, and room 1 wins the tie.

## Constraints

- `1 <= n <= 100`
- `1 <= len(meetings) <= 10^5`
- `meetings[i] = [start, end]` with `0 <= start < end <= 5 * 10^5`
- All `start` values are distinct.
