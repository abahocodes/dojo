# Approach: in-order traversal, stopped early

In-order traversal of a binary search tree visits values in sorted order. So
the `k`-th node it visits holds the `k`-th smallest value. An explicit stack
lets us pause after exactly `k` visits and also avoids deep recursion on a
skewed tree.

```python
def kth_smallest(root, k):
    stack = []
    node = root
    while True:
        while node:              # walk as far left as possible
            stack.append(node)
            node = node.left
        node = stack.pop()       # next value in sorted order
        k -= 1
        if k == 0:
            return node.val
        node = node.right
```

## Complexity

- Time: O(h + k), where `h` is the height of the tree: we descend to the
  smallest value, then do `k` steps of the traversal.
- Space: O(h) for the stack (O(n) for a skewed tree).

## Pitfalls

- `k` is 1-based: the smallest value is `k = 1`, not `k = 0`.
- Collecting all values into a list and sorting works but ignores the BST
  property and costs O(n log n).
- A recursive traversal can exceed the recursion limit on a tree that is
  thousands of nodes deep, such as one built from sorted insertions.
- Remember to keep going right after popping a node; otherwise you only visit
  the left spine.
