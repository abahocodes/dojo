You are given the `root` of a binary tree. Group its values by depth: the first
group holds the root, the second holds every node one step below the root, and
so on.

Return the groups as a list of lists, from the top level down. Inside each
group, list the values from **left to right**. An empty tree yields `[]`.

## Example 1

```
root   = [10, 6, 15, 3, null, 12, 20, null, 4]
output = [[10], [6, 15], [3, 12, 20], [4]]
```

## Example 2

```
root   = [1, null, 2, null, 3]
output = [[1], [2], [3]]     # each level holds a single node
```

## Constraints

- The tree has between `0` and `2000` nodes.
- `-1000 <= node.val <= 1000`
- The tree is given in level order; `null` marks a missing child.
