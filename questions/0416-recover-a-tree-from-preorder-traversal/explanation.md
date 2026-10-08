# Approach: a stack holding the current path

In a preorder listing, the parent of a node at depth `d` is the latest node
seen at depth `d - 1`. If we keep a stack holding the path from the root to
the last node read, then `stack[d - 1]` is that parent: anything deeper
belongs to subtrees that are now finished.

For each `(depth, value)` pair:

1. Trim the stack to length `depth`.
2. The top (if any) is the parent. Its left slot is filled first, because
   preorder lists the left subtree first and an only child is a left child.
3. Push the new node.

```python
def recover_from_preorder(traversal):
    stack = []
    i, n = 0, len(traversal)
    while i < n:
        depth = 0
        while traversal[i] == "-":
            depth += 1
            i += 1
        value = 0
        while i < n and traversal[i] != "-":
            value = value * 10 + ord(traversal[i]) - 48
            i += 1
        node = TreeNode(value)
        del stack[depth:]
        if stack:
            parent = stack[-1]
            if parent.left is None:
                parent.left = node
            else:
                parent.right = node
        stack.append(node)
    return stack[0]
```

## Complexity

- Time: O(L), where L is the length of the string. Each node is pushed and
  popped at most once.
- Space: O(h) for the stack, where h is the height.

## Pitfalls

- Values can have several digits (up to 10); read until the next dash, not a
  single character.
- A dash run marks the depth of the node that follows it. It is not a
  separator to split on blindly: `"1--2"` splits into an empty token.
- Return `stack[0]`, the root, not the top of the stack.
