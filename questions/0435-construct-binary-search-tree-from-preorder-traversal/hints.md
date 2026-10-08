# Hints

## Hint 1
The first value is the root. The values that follow and are smaller than it
form the left subtree; the rest form the right subtree. That gives an
O(n log n) or O(n^2) divide-and-conquer. Can you place each value as you read
it instead?

## Hint 2
Keep a stack of nodes whose right child may still be filled in, with values
decreasing from bottom to top. A new value smaller than the top must be the
top's left child.

## Hint 3
If the new value is larger than the top, pop while the top is smaller than the
value; the last node popped is its parent, and the new node is that parent's
right child. Push the new node in both cases.
