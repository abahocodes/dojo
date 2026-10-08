# Approach: post-order evaluation

This is an expression tree. Each operator needs its operands first, so we
evaluate in post-order: children before parents. Leaves are their own values.

The recursive version is three lines; the version below uses an explicit stack
so that very deep trees cannot overflow the call stack. Each node is pushed
once with `children_done = False` (to schedule its children) and once more
with `True` (to combine their results).

```python
def evaluate_tree(root):
    value = {}
    stack = [(root, False)]
    while stack:
        node, children_done = stack.pop()
        if node.left is None:
            value[node] = node.val == 1
        elif children_done:
            left, right = value[node.left], value[node.right]
            value[node] = (left or right) if node.val == 2 else (left and right)
        else:
            stack.append((node, True))
            stack.append((node.left, False))
            stack.append((node.right, False))
    return value[root]
```

## Complexity

- Time: O(n): each node is pushed at most twice.
- Space: O(n) for the value map, plus O(h) for the stack.

## Pitfalls

- Treating `2` and `3` as numeric values: they are operator codes, only
  meaningful on nodes with children.
- Mixing up the codes: `2` is OR and `3` is AND.
- Testing `node.right` alone to detect a leaf works only because the tree is
  full; checking that a node has no children is clearer.
