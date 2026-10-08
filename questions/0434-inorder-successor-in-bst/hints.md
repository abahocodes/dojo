# Hints

## Hint 1
You do not need the full inorder sequence. Think about where values larger than
`p` can be found relative to the search path for `p`.

## Hint 2
Start at the root. If the current value is greater than `p`, it is a candidate
answer, and anything better (smaller but still above `p`) is in its left
subtree. Otherwise, the answer must be in its right subtree.

## Hint 3
Keep `answer = -1`. Loop while the node exists: if `node.val > p`, set
`answer = node.val` and go left; else go right. Return `answer`.
