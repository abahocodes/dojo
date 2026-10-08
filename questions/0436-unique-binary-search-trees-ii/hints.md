# Hints

## Hint 1
Pick which value sits at the root. Once the root is `v`, which values must end
up in its left subtree, and which in its right?

## Hint 2
The left subtree is any valid BST on `1..v-1` and the right subtree any valid
BST on `v+1..n`. The two choices are independent, so every pair combines into
a distinct tree.

## Hint 3
Write `build(lo, hi)` returning all BSTs on the range `lo..hi`. An empty range
returns `[None]` (one empty tree), not `[]`. For each root `v` in the range,
loop over `build(lo, v-1)` x `build(v+1, hi)` and create a new root node per
pair. Memoize on `(lo, hi)` to avoid rebuilding the same ranges.
