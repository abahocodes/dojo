# Approach: carry the prefix value down the tree

Reading a path from the root is reading a binary number from its most
significant bit. Appending one bit `b` to a number `p` produces `2 * p + b`,
so each node can compute the value of the path ending at it from its parent's
value in O(1). At every leaf, that value is one complete number to add.

```python
def sum_root_to_leaf(root):
    total = 0
    stack = [(root, 0)]
    while stack:
        node, prefix = stack.pop()
        value = prefix * 2 + node.val
        if node.left is None and node.right is None:
            total += value
            continue
        if node.left:
            stack.append((node.left, value))
        if node.right:
            stack.append((node.right, value))
    return total
```

The traversal order does not matter because addition is commutative; a
recursive DFS returning the sum of both subtrees works just as well when the
tree is shallow.

## Complexity

- Time: O(n): every node is visited once.
- Space: O(h) for the stack, where `h` is the tree's height.

## Pitfalls

- Adding a value at nodes with only one child: only leaves end a path.
- Building strings and converting them at each leaf costs O(h) per leaf
  instead of O(1).
- Visiting the missing child of a one-child node as if it were a leaf with
  value `0`, which counts phantom paths.
