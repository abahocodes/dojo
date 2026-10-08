You are given the `root` of a non-empty binary tree. Peel the tree like an
onion: in each round, collect the values of every node that is **currently** a
leaf, then remove all of those leaves from the tree. Nodes that lose all their
children become leaves for the next round. Keep going until the tree is empty.

Return the collected groups in the order the rounds happened. Inside a group,
list the values **from left to right**, in the order the nodes appear in an
in-order traversal of the original tree.

## Example 1

```
root   = [1, 2, 3, 4, 5, null, 6, null, null, 7]
output = [[4, 7, 6], [5, 3], [2], [1]]
```

The first round removes the leaves `4`, `7` and `6`. That leaves `5` and `3`
childless, so they go next, then `2`, and finally the root `1`.

## Example 2

```
root   = [8, null, 4, 2, 9]
output = [[2, 9], [4], [8]]
```

## Constraints

- The tree has between `1` and `100` nodes.
- `-100 <= node.val <= 100` (values may repeat)
- The tree is given in level order; `null` marks a missing child.
