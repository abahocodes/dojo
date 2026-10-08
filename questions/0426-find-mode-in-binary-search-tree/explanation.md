# Approach: in-order runs

An in-order traversal of this BST yields the values in non-decreasing order,
so equal values form one contiguous run. Walk the tree in order and keep:

- `prev` and `count`: the value of the current run and its length so far;
- `best`: the longest run length seen;
- `modes`: the values whose run reached `best`.

When the current run reaches a new maximum, the old modes are discarded.
When it ties the maximum, its value is appended. Because values arrive in
ascending order, `modes` is sorted automatically.

```python
def find_mode(root: "TreeNode") -> list[int]:
    # Inorder visits equal values consecutively, so a running count suffices.
    modes = []
    best = 0
    count = 0
    prev = None
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        count = count + 1 if node.val == prev else 1
        prev = node.val
        if count > best:
            best = count
            modes = [node.val]
        elif count == best:
            modes.append(node.val)
        node = node.right
    return modes
```

## Complexity

- Time: `O(n)`.
- Space: `O(h)` for the traversal stack, plus the output. No hash map is
  needed. (A Morris traversal would also remove the stack.)

## Pitfalls

- Duplicates do not have to be parent and child: a copy can sit deep in the
  other subtree. Counting only along parent-child edges misses those.
- A value can be appended while its run is still growing, for example when its
  run first ties `best` and then exceeds it. That is fine: the `count > best`
  branch resets `modes` at that point.
- Return the values in ascending order. An in-order walk gives this for free,
  but a hash-map solution must sort its result.
