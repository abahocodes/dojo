# Approach: reversed postorder with a stack

Reversed, `postorder` reads root, right subtree, left subtree: a mirrored
preorder. Reversed, `inorder` reads right subtree, root, left subtree. Walk
both from the back.

Keep a stack of nodes whose left side is still open. For each new value:

- If the stack top is not `inorder[i]`, the top still has nodes in its right
  subtree to place, so the new node is its **right** child.
- Otherwise the top's right subtree is finished. Pop while the top equals
  `inorder[i]`, moving `i` left each time; the last popped node is the one
  whose **left** child the new node becomes.

Push the new node and continue.

```python
def build_tree_in_post(inorder, postorder):
    root = TreeNode(postorder[-1])
    stack = [root]
    i = len(inorder) - 1
    for value in reversed(postorder[:-1]):
        node = TreeNode(value)
        parent = stack[-1]
        if parent.val != inorder[i]:
            parent.right = node
        else:
            while stack and stack[-1].val == inorder[i]:
                parent = stack.pop()
                i -= 1
            parent.left = node
        stack.append(node)
    return root
```

**Alternative (recursive divide and conquer):** map each value to its index in
`inorder`. `build(lo, hi)` pops the next value off the end of `postorder` as
the root, splits at its inorder index, and builds the **right** half first
(because you are consuming postorder from the back), then the left. It is O(n)
too, but its recursion depth equals the tree height (up to 3000).

## Complexity

- Time: O(n): every node is pushed and popped once.
- Space: O(n) for the stack and the tree.

## Pitfalls

- In the recursive version, building the left subtree before the right one
  while consuming postorder from the end produces the wrong tree.
- Searching `inorder` linearly for each root makes the recursive version
  O(n^2); use a hash map.
- Slicing lists at every level also costs O(n^2) time and memory.
