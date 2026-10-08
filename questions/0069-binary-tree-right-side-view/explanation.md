# Approach: BFS, keep the last node of every level

The node visible from the right at a given depth is the rightmost node of that
depth. A level-by-level traversal that lists each level from left to right
makes it the last element of the level.

```python
def right_side_view(root):
    if root is None:
        return []
    view = []
    level = [root]
    while level:
        view.append(level[-1].val)
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
    return view
```

**Alternative (DFS):** traverse with an explicit stack of `(node, depth)`,
visiting the right child before the left. The first node reached at each new
depth (`depth == len(view)`) is the visible one.

## Complexity

- Time: O(n): every node is visited once.
- Space: O(w) for the current level, where `w` is the widest level.

## Pitfalls

- Following only right children is wrong: when a right subtree is shallower, a
  node from the left subtree becomes visible at the deeper levels (Example 1).
- Push the left child before the right one, or "last in the level" becomes the
  leftmost node.
- Return `[]` for an empty tree.
