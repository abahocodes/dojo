You are given `root`, the root of a binary tree (or `None` for an empty tree).

Return the values of all nodes in **inorder**: for every node, first everything
in its left subtree, then the node itself, then everything in its right
subtree.

Trees are written in level order, with `null` marking a missing child.

## Example 1

```
root   = [5, 3, 8, 1, 4, null, 9]

        5
       / \
      3   8
     / \   \
    1   4   9

output = [1, 3, 4, 5, 8, 9]
```

## Example 2

```
root   = [2, null, 7, 6]

    2
     \
      7
     /
    6

output = [2, 6, 7]
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-100 <= node.val <= 100`
- The tree may be very unbalanced (up to `10^4` levels deep).

**Follow-up:** the recursive solution is short. Write it, then write an
iterative version that uses an explicit stack.
