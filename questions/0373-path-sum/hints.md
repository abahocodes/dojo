# Hints

## Hint 1
Instead of tracking the sum so far, track how much is still missing: subtract
each node's value from `target_sum` as you go down.

## Hint 2
The check only happens at leaves. Values can be negative, so you cannot stop
early when the running total passes the target.

## Hint 3
Push `(root, target_sum - root.val)` on a stack. Pop a pair; if the node is a
leaf and the remainder is `0`, return `true`. Otherwise push each existing
child with `remainder - child.val`. If the stack empties, return `false`.
