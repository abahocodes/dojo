You are given the `root` of a binary tree with distinct values, and a value
`start` that is present in the tree.

At minute `0`, the node holding `start` becomes infected. Every minute after
that, the infection spreads from each infected node to all of its neighbors
that are not yet infected. A node's neighbors are its parent and its children.

Return the number of minutes it takes until **every** node of the tree is
infected.

## Example 1

```
root   = [2, 6, 9, null, 8, 4, 1, 3, null, null, null, null, 7]
start  = 4
output = 5
```

- Minute 0: `4`
- Minute 1: `9`
- Minute 2: `2` and `1`
- Minute 3: `6` and `7`
- Minute 4: `8`
- Minute 5: `3`, the last healthy node

## Example 2

```
root   = [12]
start  = 12
output = 0     # the only node is infected from the start
```

## Constraints

- The tree has between `1` and `10^5` nodes.
- `1 <= node.val <= 10^5`, and all values are distinct.
- A node with value `start` exists in the tree.
- The tree is given in level order; `null` marks a missing child.
