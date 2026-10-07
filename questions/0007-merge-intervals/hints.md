# Hints

## Hint 1
Overlaps are hard to spot when intervals are scattered. What ordering would put
intervals that might merge right next to each other?

## Hint 2
Sort by start. Now an interval can only merge with the group just before it —
and it does so exactly when its start is no larger than that group's end.

## Hint 3
After sorting, keep a result list. For each `[s, e]`: if the result is empty or
`s > result[-1][1]`, append a new interval; otherwise extend
`result[-1][1] = max(result[-1][1], e)`.
