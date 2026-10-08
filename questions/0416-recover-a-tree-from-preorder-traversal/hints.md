# Hints

## Hint 1
Split the string into `(depth, value)` pairs first: count dashes, then read
digits until the next dash.

## Hint 2
In preorder, a node at depth `d` is a child of the most recent node seen at
depth `d - 1`. What structure holds "the most recent node at each depth" on
the current root-to-node path?

## Hint 3
Keep a stack that is exactly the path from the root to the last node. For a
new node at depth `d`, shrink the stack to length `d`; the top is its parent.
Attach it as the left child if that slot is empty, otherwise as the right
child, then push it.
