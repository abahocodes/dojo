# Approach: compare every node with the root

The only value every node could share is the root's. Traverse the tree and stop
at the first node that holds something else.

```python
def is_unival_tree(root):
    value = root.val
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val != value:
            return False
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    return True
```

The recursive version checks that each child, when present, has the parent's
value and is itself univalued.

## Complexity

- Time: O(n): each node is checked at most once, and the walk stops early on a
  mismatch.
- Space: O(h) for the stack, where `h` is the tree height.

## Pitfalls

- Checking only the root's direct children: a mismatch can hide anywhere.
- Treating a missing child as a mismatch: absent nodes hold no value.
