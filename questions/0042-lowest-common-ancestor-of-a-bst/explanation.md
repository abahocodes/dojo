# Approach: walk down until p and q split

In a binary search tree, all values smaller than a node live in its left
subtree and all larger values live in its right subtree. Starting from the
root:

- if both `p` and `q` are smaller than the node, both are in the left subtree,
  and so is their lowest common ancestor;
- if both are larger, the same holds on the right;
- otherwise they are on different sides (or one of them is this node), so no
  deeper node contains both: this node is the answer.

```python
def lowest_common_ancestor(root, p, q):
    lo, hi = min(p, q), max(p, q)
    node = root
    while True:
        if hi < node.val:
            node = node.left
        elif lo > node.val:
            node = node.right
        else:
            return node.val
```

The general binary-tree technique (record parents, collect the ancestors of
`p`, then climb from `q` until you hit one) also works, but it visits the whole
tree and ignores the ordering.

## Complexity

- Time: O(h), where `h` is the height of the tree: O(log n) when balanced,
  O(n) when skewed.
- Space: O(1) with the loop (a recursive version uses O(h) stack).

## Pitfalls

- Forgetting that a node can be its own ancestor: when `p` equals the current
  node, stop there.
- Assuming `p < q`: normalize with `min`/`max` or compare both explicitly.
- Searching the whole tree like an unordered binary tree, which is correct but
  wastes the BST property.
