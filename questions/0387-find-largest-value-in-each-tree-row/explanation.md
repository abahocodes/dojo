# Approach: level-by-level BFS

Process the tree one level at a time. Keep the current level as a list; its
maximum goes into the answer, and its children (left to right) form the next
level. Stop when a level is empty.

```python
def largest_values(root):
    result = []
    level = [root] if root else []
    while level:
        result.append(max(node.val for node in level))
        next_level = []
        for node in level:
            if node.left:
                next_level.append(node.left)
            if node.right:
                next_level.append(node.right)
        level = next_level
    return result
```

**Alternative (DFS):** traverse with the depth; when `depth == len(result)`
append the node's value, otherwise set `result[depth] = max(result[depth],
node.val)`.

## Complexity

- Time: O(n): each node is handled once.
- Space: O(w) for the current and next level, where `w` is the widest level,
  plus the output.

## Pitfalls

- Starting each level's maximum at `0`: a level of negative values would
  report `0`. Use the first node's value or the smallest 32-bit integer.
- Returning `[0]` or `[[]]` instead of `[]` for an empty tree.
- Overflow when comparing with a sentinel computed as `INT_MIN - 1` or
  similar.
