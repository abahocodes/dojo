# Hints

## Hint 1
Reversing one group is the classic in-place list reversal, run for exactly
`k` nodes. The tricky part is reconnecting each reversed group to what comes
before and after it.

## Hint 2
Use a sentinel node in front of the head. Keep a pointer `group_prev` to the
node just before the current group. Before reversing, check that `k` more nodes
actually exist; if not, stop.

## Hint 3
For a full group starting at `first = group_prev.next` and ending at `kth`,
reverse the `k` nodes with `prev` initialised to `kth.next` (so the old first
node ends up pointing past the group). Then set `group_prev.next = kth` and
`group_prev = first`, and repeat.
