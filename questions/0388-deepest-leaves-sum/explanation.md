# Approach: breadth-first search, keep the last level's sum

A level-order traversal visits the tree one level at a time, so the deepest
level is simply the last one it produces. Build each next level from the
children of the current one; when there are no children left, the current
level is the deepest and its sum is the answer.

```python
def deepest_leaves_sum(root):
    level = [root]
    while True:
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        if not nxt:
            return sum(node.val for node in level)
        level = nxt
```

**Alternative (DFS):** walk the tree with an explicit stack of
`(node, depth)` pairs, tracking the maximum depth seen and the running sum at
that depth. When a deeper node appears, reset the sum to its value; when a node
at the current maximum depth appears, add to the sum.

## Complexity

- Time: O(n): every node is visited once.
- Space: O(w), where `w` is the width of the widest level (the DFS variant uses
  O(h) for the height instead).

## Pitfalls

- Summing every leaf: leaves on shallower levels (like `9` in Example 1) must
  be ignored.
- Recursive DFS can overflow the call stack on a 10^4-node chain in some
  languages; the level-by-level loop has no such problem.
