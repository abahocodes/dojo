# Hints

## Hint 1
Comparing the tree with itself, two subtrees `x` and `y` are mirrors when their
roots have equal values, `x.left` mirrors `y.right`, and `x.right` mirrors
`y.left`.

## Hint 2
Start by checking that `root.left` and `root.right` are mirrors. Both missing
counts as a match; exactly one missing is a mismatch.

## Hint 3
To avoid recursion, keep a stack (or queue) of pairs, starting with
`(root.left, root.right)`. Pop a pair: skip it if both are missing, return
`false` if only one is or the values differ, otherwise push
`(a.left, b.right)` and `(a.right, b.left)`. If the stack empties, return
`true`.
