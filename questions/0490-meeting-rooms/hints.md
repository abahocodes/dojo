# Hints

## Hint 1
Checking every pair of meetings works but is quadratic. Is there an order in
which a conflict, if one exists, must show up between neighbours?

## Hint 2
Sort the meetings by start time. If some meeting overlaps a later one, it
also overlaps the meeting that comes right after it in this order.

## Hint 3
After sorting, scan once and return `false` as soon as
`intervals[i][0] < intervals[i - 1][1]`. Use strict `<`: touching meetings are
fine.
