You are given `root`, the root of a binary tree (or `None` for an empty tree).

Return the values of all nodes in **postorder**: for every node, first
everything in its left subtree, then everything in its right subtree, and the
node itself last.

Trees are written in level order, with `null` marking a missing child.

## Example 1

```
root   = [5, 3, 8, 1, 4, null, 9]

        5
       / \
      3   8
     / \   \
    1   4   9

output = [1, 4, 3, 9, 8, 5]
```

## Example 2

```
root   = [2, null, 7, 6]

    2
     \
      7
     /
    6

output = [6, 7, 2]
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-100 <= node.val <= 100`
- The tree may be very unbalanced (up to `10^4` levels deep).

**Follow-up:** solve it iteratively, with an explicit stack instead of
recursion.
