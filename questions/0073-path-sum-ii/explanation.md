# Approach: depth-first search with backtracking

Walk the tree depth-first, maintaining the current root-to-node path and its
sum. When the walk reaches a leaf whose path sums to the target, record a copy
of the path. Because the left child is explored before the right one, leaves
(and so paths) are found in left-to-right order.

The reference solution uses an explicit stack, so a very deep tree can't
overflow the call stack. Each node is pushed twice: once to enter it, and once
more as a "leaving" marker that undoes its effect on `path` and `total` after
both of its subtrees are finished.

```python
def path_sum(root, target_sum):
    result = []
    if root is None:
        return result
    path, total = [], 0
    stack = [(root, False)]
    while stack:
        node, leaving = stack.pop()
        if leaving:
            path.pop()
            total -= node.val
            continue
        path.append(node.val)
        total += node.val
        stack.append((node, True))
        if node.left is None and node.right is None:
            if total == target_sum:
                result.append(list(path))
        else:
            if node.right:
                stack.append((node.right, False))
            if node.left:
                stack.append((node.left, False))
    return result
```

## Complexity

- Time: O(n + L), where `L` is the total length of the returned paths. Copying
  a path costs O(h), and in the worst case O(n) leaves each copy a path of
  length O(h).
- Space: O(h) for the path and stack, plus the output.

## Pitfalls

- Values can be negative, so you can't stop early when the running sum passes
  the target.
- Only leaves end a path: a node with one child is not a leaf (Example 3).
- Appending `path` itself instead of a copy: later backtracking mutates every
  saved path.
- Visiting the right child first returns the right paths in the wrong order.
