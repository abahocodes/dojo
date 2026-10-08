# Hints

## Hint 1
The list never needs to be read twice: one pointer walking forward is enough.

## Hint 2
After keeping `m` nodes you are standing on the last kept node. Everything you
delete next is simply skipped by changing that node's `next` pointer.

## Hint 3
Loop while `cur` exists: advance `cur` `m - 1` times (stopping if it becomes
`None`) to reach the last kept node; then walk a second pointer `n` nodes past
it and set `cur.next` to whatever follows; finally move `cur` to `cur.next`.
