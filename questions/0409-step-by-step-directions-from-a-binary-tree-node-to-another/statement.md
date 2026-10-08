You are given the `root` of a binary tree with `n` nodes whose values are
exactly the integers `1` to `n`, each used once. You are also given two
different values, `start_value` and `dest_value`, both present in the tree.

Starting at the node holding `start_value`, describe how to walk to the node
holding `dest_value` along the **shortest** path. Each step is one character:

- `'U'`: move up to the current node's parent,
- `'L'`: move down to the current node's left child,
- `'R'`: move down to the current node's right child.

Return the string of steps. In a tree the shortest path between two nodes is
unique, so the answer is unique too.

## Example 1

```
root        = [4, 7, 2, 1, null, 6, 3, null, 5]
start_value = 5
dest_value  = 6
output      = "UUURL"
```

From `5`, climb to `1`, then `7`, then the root `4`. From there go right to `2`
and left to `6`.

## Example 2

```
root        = [1, 2, 3]
start_value = 1
dest_value  = 3
output      = "R"
```

## Constraints

- `2 <= n <= 10^5`
- The node values are a permutation of `1..n`.
- `1 <= start_value, dest_value <= n` and `start_value != dest_value`.
- The tree is given in level order; `null` marks a missing child.
