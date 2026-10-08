# Approach: depth-first search with a remaining amount

Walk down from the root carrying how much of `target_sum` is still needed. At a
leaf the path is complete, and it matches when nothing is left. An explicit
stack keeps the search safe on very deep trees.

```python
def has_path_sum(root, target_sum):
    if root is None:
        return False
    stack = [(root, target_sum - root.val)]
    while stack:
        node, remaining = stack.pop()
        if node.left is None and node.right is None:
            if remaining == 0:
                return True
            continue
        if node.left:
            stack.append((node.left, remaining - node.left.val))
        if node.right:
            stack.append((node.right, remaining - node.right.val))
    return False
```

The recursive version is a one-liner per case: an empty tree is `false`, a
leaf matches when its value equals the remaining amount, and otherwise either
child may finish the job with `target_sum - root.val`.

## Complexity

- Time: O(n): each node is pushed and popped at most once.
- Space: O(h) for the stack, where `h` is the tree height.

## Pitfalls

- Accepting a match at an inner node: the path must end at a leaf.
- An empty tree is `false`, even when `target_sum` is `0`.
- Pruning when the total exceeds the target: negative values can bring it back.
- A node with one child is not a leaf; do not treat the missing side as a
  zero-length path.
