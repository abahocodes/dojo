You are given the `root` of a binary tree. Return the values of its nodes in
**preorder**: first the node itself, then everything in its left subtree (in
preorder), then everything in its right subtree (in preorder).

An empty tree returns `[]`.

## Example 1

```
root   = [8, 3, 10, 1, 6, null, 14, null, null, 4, 7]
output = [8, 3, 1, 6, 4, 7, 10, 14]
```

## Example 2

```
root   = []
output = []
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-100 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.

**Follow-up:** the recursive solution is short. Can you write it iteratively,
with an explicit stack?
