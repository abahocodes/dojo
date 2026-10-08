# Hints

## Hint 1
The answer is the largest number of meetings that are running at the same
moment. Where in time can that maximum occur?

## Hint 2
Process meetings in order of start time. When a meeting starts, can it take
over a room whose meeting has already ended? You only need to know the
earliest end time among the rooms in use.

## Hint 3
Sort the start times and the end times separately. Walk the starts; keep a
pointer into the sorted ends. If the current start is `>=` the earliest
unconsumed end, a room frees up (advance the end pointer); otherwise open a
new room. A min-heap of end times works the same way.
