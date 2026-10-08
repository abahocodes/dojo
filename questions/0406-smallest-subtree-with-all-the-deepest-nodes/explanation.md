# Approach: one post-order pass returning (height, answer)

For any node, compare the heights of its two subtrees:

- If the left subtree is taller, every deepest node below this node is in the
  left subtree, so the answer is whatever the left subtree's answer is.
- Symmetrically for the right.
- If they tie, the deepest nodes appear on both sides, and the only node
  whose subtree holds both groups (with nothing lower doing so) is the node
  itself.

Each call returns both pieces of information, so the whole tree is solved in a
single pass:

```python
def subtree_with_all_deepest(root):
    def dfs(node):
        if node is None:
            return 0, None
        left_h, left_ans = dfs(node.left)
        right_h, right_ans = dfs(node.right)
        if left_h > right_h:
            return left_h + 1, left_ans
        if right_h > left_h:
            return right_h + 1, right_ans
        return left_h + 1, node

    return dfs(root)[1]
```

**Alternative (BFS + parents):** do a level-order traversal that records each
node's parent and keeps the last level. Then repeatedly replace that set of
deepest nodes with the set of their parents until only one node remains. That
node is the answer. It also runs in O(n) and avoids recursion.

## Complexity

- Time: O(n), each node is visited once.
- Space: O(h) for the recursion, where `h <= 500` here.

## Pitfalls

- Returning the parent of the deepest nodes instead of their LCA: when there
  is a single deepest node, the answer is that node, not its parent.
- Computing heights separately at every node (calling a `height()` helper
  inside the DFS) is O(n^2) on path-shaped trees.
- Comparing depths of the *answers* instead of the heights of the subtrees:
  what decides the direction is how deep each side goes.
