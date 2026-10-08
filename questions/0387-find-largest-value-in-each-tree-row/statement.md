You are given the `root` of a binary tree. Number the levels from the top:
the root is on depth `0`, its children on depth `1`, and so on.

Return a list whose `i`-th element is the **largest** value among the nodes on
depth `i`, from the top level down. An empty tree yields `[]`.

## Example 1

```
root   = [4, 9, 2, 3, null, 7, -1]
output = [4, 9, 7]
```

## Example 2

```
root   = []
output = []
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-2^31 <= node.val <= 2^31 - 1`
- The tree is given in level order; `null` marks a missing child.
