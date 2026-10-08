# Approach: collect the leaves with a depth-first traversal

A depth-first traversal that always handles the left subtree before the right
one reaches the leaves in left-to-right order. Gather each tree's leaf values
into a list and compare the lists.

An explicit stack avoids recursion: pushing the right child before the left
child means the left child is popped (and explored) first.

```python
def leaf_similar(root1, root2):
    def leaves(root):
        out = []
        stack = [root] if root else []
        while stack:
            node = stack.pop()
            if node.left is None and node.right is None:
                out.append(node.val)
                continue
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)
        return out

    return leaves(root1) == leaves(root2)
```

A lazier variant walks both trees at the same time with two generators and
stops at the first mismatch, which saves memory when the sequences differ
early.

## Complexity

- Time: O(n1 + n2): every node of both trees is visited once.
- Space: O(h1 + h2) for the stacks plus O(l1 + l2) for the leaf lists, where
  `h` is a tree's height and `l` its number of leaves.

## Pitfalls

- Treating a node with one child as a leaf: a leaf has *no* children.
- Pushing the left child before the right one onto a stack, which reverses
  the order.
- Comparing only the sets or the sums of leaf values: order and length both
  matter (`[1, 2]` is not `[2, 1]`, and `[1]` is not `[1, 1]`).
