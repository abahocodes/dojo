You are given the `root` of a binary tree. The **height** of a subtree is the
number of nodes on its longest downward path; an empty subtree has height `0`.

Call the tree **balanced** when, at *every* node, the heights of the left and
right subtrees differ by at most `1`. Return `true` if the tree is balanced and
`false` otherwise. An empty tree is balanced.

## Example 1

```
root   = [3, 1, 5, null, 2]
output = true
```

## Example 2

```
root   = [1, 2, 3, 4, null, null, null, 5]
output = false   # at node 2: left height 2, right height 0
```

## Constraints

- The tree has between `0` and `3000` nodes.
- `-10^4 <= node.val <= 10^4`
- The tree is given in level order; `null` marks a missing child.
