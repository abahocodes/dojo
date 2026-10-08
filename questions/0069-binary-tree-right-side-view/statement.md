You are given the `root` of a binary tree. Picture yourself standing to the
right of the tree and looking at it: at each depth, you can see only the
rightmost node of that depth, and it hides everything to its left.

Return the values you can see, ordered from the root's depth down to the
deepest level. An empty tree yields `[]`.

## Example 1

```
root   = [1, 2, 3, 4, null, null, null, 7]
output = [1, 3, 4, 7]   # 4 and 7 stick out below the right side
```

## Example 2

```
root   = [8, 3, null, 1, 6]
output = [8, 3, 6]
```

## Constraints

- The tree has between `0` and `3000` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
