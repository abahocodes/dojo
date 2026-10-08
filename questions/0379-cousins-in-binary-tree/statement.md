You are given the `root` of a binary tree whose node values are all distinct,
and two different values `x` and `y` that both appear in the tree.

The **depth** of a node is its distance from the root (the root has depth 0).
Two nodes are **cousins** when they have the same depth but different parents.
Siblings, which share a parent, are not cousins.

Return `true` if the nodes holding `x` and `y` are cousins, and `false`
otherwise.

## Example 1

```
root   = [1, 2, 3, null, 4, 5]
x      = 4
y      = 5
output = true      # both at depth 2; 4's parent is 2, 5's parent is 3
```

## Example 2

```
root   = [1, 2, 3, 4, 5]
x      = 4
y      = 5
output = false     # same depth, but both are children of 2
```

## Constraints

- The tree has between `2` and `100` nodes.
- `1 <= node.val <= 100`, and all values are distinct.
- `x != y`, and both values are in the tree.
- The tree is given in level order; `null` marks a missing child.
