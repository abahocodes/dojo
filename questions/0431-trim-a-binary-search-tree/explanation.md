# Approach: fix the root, then the two spines

Recursively, the rule is short: a node below `low` is replaced by the trimmed
version of its right subtree, a node above `high` by the trimmed version of its
left subtree, and an in-range node keeps itself and trims both children. This
is unique because every surviving node keeps its nearest surviving ancestor.

To avoid deep recursion we can exploit the BST ordering:

1. Move `root` until it lands in `[low, high]`: go right while it is too small,
   left while it is too large. If it falls off the tree, nothing survives.
2. Everything in the new root's left subtree is smaller than the root, hence
   below `high`; only `low` can cut there. Walk down: if `node.left` is too
   small, splice in its right subtree (`node.left = node.left.right`) and look
   again; otherwise step to `node.left`.
3. Mirror this on the right side, where only `high` can cut.

```python
def trim_bst(root, low, high):
    while root is not None and (root.val < low or root.val > high):
        root = root.right if root.val < low else root.left
    if root is None:
        return None

    node = root
    while node.left is not None:
        if node.left.val < low:
            node.left = node.left.right
        else:
            node = node.left

    node = root
    while node.right is not None:
        if node.right.val > high:
            node.right = node.right.left
        else:
            node = node.right
    return root
```

## Complexity

- Time: O(n) in the worst case (each node is stepped over or spliced at most
  once); often much less, since only the two spines are walked.
- Space: O(1) extra (the recursive version uses O(h) stack).

## Pitfalls

- Dropping a whole subtree when its root is out of range: a node below `low`
  can still have in-range descendants on its right.
- After splicing `node.left = node.left.right`, the new left child may also be
  too small, so check again before moving on.
- Recursion depth on a tree that is one long chain.
