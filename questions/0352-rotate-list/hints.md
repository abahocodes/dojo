# Hints

## Hint 1
Rotating a list of length `n` by `n` places gives back the same list. So only
`k mod n` matters — never simulate `k` rotations one by one.

## Hint 2
After rotating by `r = k mod n`, the last `r` nodes move to the front in their
original order. Which node becomes the new tail?

## Hint 3
Find the length and the tail in one pass. If `r == 0`, return `head`. Otherwise
connect the tail to `head` (forming a ring), walk `n - r - 1` steps from `head`
to the new tail, take `new_head = new_tail.next`, and cut the ring with
`new_tail.next = None`.
