You are given the `root` of a binary tree. Turn the tree into its mirror image:
at every node, the left subtree and the right subtree trade places. Return the
root of the mirrored tree (an empty tree stays empty).

## Example 1

```
root   = [8, 3, 12, 1, 5, 10, 14]
output = [8, 12, 3, 14, 10, 5, 1]
```

```
       8                 8
     /   \             /   \
    3     12    ->    12     3
   / \   /  \        /  \   / \
  1   5 10  14      14  10 5   1
```

## Example 2

```
root   = [4, 2, null, 1]
output = [4, null, 2, null, 1]     # a left chain becomes a right chain
```

## Constraints

- The tree has between `0` and `3000` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
