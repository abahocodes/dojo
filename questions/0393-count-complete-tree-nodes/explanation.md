# Approach: peel off perfect subtrees

Let `lh` be the depth of the leftmost path in the root's left subtree and `rh`
the same for the right subtree.

- If `lh == rh`, the bottom level extends into the right subtree, so the left
  subtree is perfect with height `lh`: it has `2^lh - 1` nodes. Together with
  the root that is `2^lh` nodes; continue counting in the right subtree.
- Otherwise `rh == lh - 1` and the bottom level stops inside the left subtree,
  so the right subtree is perfect with height `rh`: count `2^rh` (including the
  root) and continue in the left subtree.

Each subtree we continue into is itself complete, so the same reasoning
applies all the way down.

```python
def count_nodes(root):
    def left_depth(node):
        depth = 0
        while node:
            depth += 1
            node = node.left
        return depth

    count = 0
    node = root
    while node:
        lh = left_depth(node.left)
        rh = left_depth(node.right)
        if lh == rh:
            count += 1 << lh
            node = node.right
        else:
            count += 1 << rh
            node = node.left
    return count
```

**Equivalent variant:** at each node compare the leftmost and the rightmost
depth; if they are equal the subtree is perfect and has `2^h - 1` nodes,
otherwise return `1 + count(left) + count(right)`. Only one of the two
recursive calls can be non-perfect, so this is also O(log^2 n).

**Brute force:** any traversal counts the nodes in O(n).

## Complexity

- Time: O(log^2 n): O(log n) steps, each measuring a depth in O(log n).
- Space: O(1) extra.

## Pitfalls

- Applying the trick to a tree that is not complete: it relies on the left
  packing of the last level.
- Off-by-one in the perfect-tree size: a perfect tree of height `h` has
  `2^h - 1` nodes, and adding the current root gives exactly `2^h`.
- Forgetting the empty tree, which has `0` nodes.
