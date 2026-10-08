You are given the `root` of a binary tree. The **tilt** of a node is the
absolute difference between the sum of all values in its left subtree and the
sum of all values in its right subtree. A missing subtree has sum `0`, so a
leaf's tilt is `0`.

Return the sum of the tilts of **every** node in the tree. An empty tree has
total tilt `0`.

## Example 1

```
root   = [5, 3, -2, 1, 4]
output = 13
# node 3:  |1 - 4|           = 3
# node 5:  |(3+1+4) - (-2)|  = 10
# leaves 1, 4 and -2 have tilt 0
```

## Example 2

```
root   = []
output = 0
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-1000 <= node.val <= 1000`
- The answer fits in a signed 32-bit integer.
- The tree is given in level order; `null` marks a missing child.
