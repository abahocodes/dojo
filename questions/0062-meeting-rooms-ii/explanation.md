# Approach: sweep sorted starts against sorted ends

The rooms needed equal the most meetings in progress at any one time. Sort the
start times and the end times independently. Walk through the starts in order;
`ends[j]` is the earliest end time not yet used to free a room.

- If `start >= ends[j]`, some meeting is over by now: reuse its room and move
  `j` forward.
- Otherwise every room is still busy: open a new one.

Which meeting a given end belongs to doesn't matter, only how many have ended.

```python
def min_meeting_rooms(intervals):
    starts = sorted(s for s, _ in intervals)
    ends = sorted(e for _, e in intervals)
    rooms = j = 0
    for s in starts:
        if s >= ends[j]:
            j += 1          # a room frees up; reuse it
        else:
            rooms += 1      # everyone is busy; open another room
    return rooms
```

**Alternative (min-heap):** sort meetings by start, keep a min-heap of the end
times of rooms in use. For each meeting, pop the top if it ended by `start`,
then push this meeting's end. The heap's final size is the answer.

## Complexity

- Time: O(n log n) for the sorts; the sweep is O(n).
- Space: O(n) for the two sorted lists.

## Pitfalls

- Use `>=`, not `>`: a meeting that ends at `10` frees its room for one
  starting at `10`.
- Counting overlapping pairs is not the answer: in `[[1, 10], [2, 3], [4, 5]]`
  the long meeting overlaps both others, yet two rooms suffice because
  `[2, 3]` and `[4, 5]` can share one.
- In JavaScript, `sort()` without a comparator sorts numbers as strings.
