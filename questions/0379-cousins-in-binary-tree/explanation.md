# Approach: breadth-first search, recording parents level by level

Cousins sit on the same level, so a level-by-level search is a natural fit.
Expand every node of the current level and, for each child, check whether it
holds `x` or `y`; the node being expanded is that child's parent. Once the
level is done, the answer is decided as soon as at least one value has shown up.

```python
def is_cousins(root, x, y):
    level = [root]
    while level:
        parent_x = parent_y = None
        next_level = []
        for node in level:
            for child in (node.left, node.right):
                if child is None:
                    continue
                if child.val == x:
                    parent_x = node
                elif child.val == y:
                    parent_y = node
                next_level.append(child)
        if parent_x and parent_y:
            return parent_x is not parent_y
        if parent_x or parent_y:
            return False
        level = next_level
    return False
```

If either value is the root, it has no parent and nothing else shares its
depth, so the search never finds both on one level and returns `false`.

## Complexity

- Time: O(n): each node is expanded at most once.
- Space: O(w) for the current and next levels, where `w` is the widest level.

## Pitfalls

- Siblings: same depth, same parent, so the answer is `false`.
- Returning `true` when one value is the root: the root has no parent and
  no other node shares its depth.
- Forgetting that one value may appear on a level without the other: then
  they have different depths and the answer is `false`.
