# Hints

## Hint 1
The definition is already a recursive algorithm: find the maximum, recurse
on both sides. That is O(n^2) in the worst case (a sorted array). Can you do
better?

## Hint 2
Scan left to right. For a new value `x`, its left child is the largest of the
consecutive smaller values just before it, and its parent is either the
nearest larger value on its left or the nearest larger value on its right
(whichever is smaller). "Nearest larger" suggests a monotonic stack.

## Hint 3
Keep a stack of nodes with decreasing values. For each `x`, pop every node
smaller than `x`; the last one popped becomes `x`'s left child. If the stack
is not empty afterwards, `x` becomes the right child of its top. Push `x`. At
the end, the bottom of the stack is the root.
