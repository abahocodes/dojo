# Approach: split by the root, or one pass with a stack

**Divide and conquer.** `preorder[0]` is the root. Its position `m` in
`inorder` splits `inorder` into the left subtree (`m` values) and the right
subtree. In `preorder`, the root is followed by those `m` left-subtree values
and then the right-subtree values. Recurse on both parts, using a dictionary of
inorder positions so each split costs O(1). This is O(n), but its recursion is
as deep as the tree.

**One pass with a stack** (the reference solution) gives the same result
without recursion. Read `preorder` from left to right. The stack holds the path
of nodes whose right child is not decided yet, and `j` points at the next value
in `inorder`:

- If the stack top is not `inorder[j]`, the top's left subtree is still being
  built, so the new value is its left child.
- Otherwise the left subtrees of the top nodes are complete. Pop while the top
  equals `inorder[j]`, advancing `j`. The new value is the right child of the
  last node popped.

```python
def build_tree(preorder, inorder):
    if not preorder:
        return None
    root = TreeNode(preorder[0])
    stack = [root]
    j = 0
    for val in preorder[1:]:
        node = stack[-1]
        if node.val != inorder[j]:
            node.left = TreeNode(val)
            stack.append(node.left)
        else:
            while stack and stack[-1].val == inorder[j]:
                node = stack.pop()
                j += 1
            node.right = TreeNode(val)
            stack.append(node.right)
    return root
```

## Complexity

- Time: O(n): each node is pushed and popped once, and `j` only moves forward.
- Space: O(h) for the stack, where `h` is the height (O(n) for a skewed tree).

## Pitfalls

- Searching `inorder` linearly for each root makes the recursive version
  O(n^2) on a skewed tree. Use a value-to-index map.
- Slicing lists at each recursive call also costs O(n) per level. Pass index
  ranges instead.
- Recursion on a chain of thousands of nodes can hit the recursion limit.
- Handle empty input: return `None`, not a node.
