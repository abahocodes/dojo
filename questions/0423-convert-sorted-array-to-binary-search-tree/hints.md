# Hints

## Hint 1
The root of a BST built from a sorted list splits the list into the values that
go left and the values that go right. Which element keeps the two sides as
equal in size as possible?

## Hint 2
Choose the middle element as the root. Then build the left subtree from the
elements before it and the right subtree from the elements after it, in exactly
the same way.

## Hint 3
Write `build(lo, hi)`: return `None` when `lo > hi`. Otherwise take
`mid = (lo + hi) // 2` and return a node with value `nums[mid]`, left child
`build(lo, mid - 1)` and right child `build(mid + 1, hi)`. Pass indices
instead of slicing the list.
