# Approach: find the unique root, then merge while validating

Each merge consumes one tree, and only a tree whose root value matches a leaf
can be consumed. The tree that survives is the one never consumed, so its root
value must not appear as a leaf anywhere. If exactly one root has that
property, it is the only candidate for the final root. Otherwise the answer is
empty.

Put every other tree in a hash map keyed by root value. Then run an iterative
DFS from the candidate root, carrying an open interval `(lo, hi)` that each
node's value must lie in. When the DFS reaches a leaf whose value is a key in
the map, remove that tree from the map and graft its children onto the leaf.
Grafting there is the only option: a value can appear only once in a valid
BST, so if two leaves share a value the tree fails the bounds check either way.

The answer is valid when no node broke its bounds and the map is empty. A
non-empty map means some trees were never reached: they form a separate
component or a cycle.

```python
def can_merge(trees):
    roots = {t.val: t for t in trees}
    leaf_values = set()
    for t in trees:
        for child in (t.left, t.right):
            if child is not None:
                leaf_values.add(child.val)

    candidates = [t for t in trees if t.val not in leaf_values]
    if len(candidates) != 1:
        return None
    root = candidates[0]
    del roots[root.val]

    stack = [(root, 0, 1 << 31)]
    while stack:
        node, lo, hi = stack.pop()
        if not lo < node.val < hi:
            return None
        if node.left is None and node.right is None and node.val in roots:
            sub = roots.pop(node.val)
            node.left, node.right = sub.left, sub.right
        if node.left is not None:
            stack.append((node.left, lo, node.val))
        if node.right is not None:
            stack.append((node.right, node.val, hi))

    return root if not roots else None
```

## Complexity

- Time: O(n). Each tree is put in the map once and each node is visited once.
- Space: O(n) for the map, the leaf set and the stack.

## Pitfalls

- Checking each tree locally is not enough. The bounds come from every
  ancestor, so `9` under `3` can be fine in `[3, 1, 9]` and still break the
  final tree when `3` hangs to the left of `8`.
- Forgetting the final emptiness check accepts inputs where some trees form a
  cycle (`[[2, 1], [1, 2]]` has no root at all; a cycle off to the side leaves
  trees in the map).
- Merged trees can form a chain about `n` levels deep. Recursion overflows the
  stack in Python and can in other languages too, so use an explicit stack.
- Use strict comparisons: duplicate values make the BST invalid.
