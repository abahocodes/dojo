You are given the `root` of a binary tree. Return its values level by level,
from the top level down, but alternate the reading direction: level `0` (the
root) is read **left to right**, level `1` **right to left**, level `2` left to
right again, and so on.

Each level becomes one list in the output. An empty tree returns `[]`.

## Example 1

```
root   = [10, 6, 15, 3, null, 12, 20, null, 4]
output = [[10], [15, 6], [3, 12, 20], [4]]
```

## Example 2

```
root   = [1, 2, 3, 4, 5, 6, 7]
output = [[1], [3, 2], [4, 5, 6, 7]]
```

## Constraints

- The tree has between `0` and `2000` nodes.
- `-100 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.
