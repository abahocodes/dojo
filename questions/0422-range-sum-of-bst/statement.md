You are given the `root` of a binary search tree with distinct values, and two
integers `low` and `high`. Return the sum of the values of every node whose value
lies in the closed range `[low, high]`. If no value falls in the range, the sum
is `0`.

Use the ordering of the tree: a subtree that cannot contain a value in the
range should not be visited.

## Example 1

```
root   = [10, 5, 15, 3, 7, null, 18]
low    = 7
high   = 15
output = 32      # 7 + 10 + 15
```

## Example 2

```
root   = [10, 5, 15, 3, 7, 13, 18, 1, null, 6]
low    = 6
high   = 10
output = 23      # 6 + 7 + 10
```

## Constraints

- The tree has between `1` and `2 * 10^4` nodes.
- `1 <= node.val <= 10^5`, and all values are distinct.
- `1 <= low <= high <= 10^5`
- The answer fits in a 32-bit signed integer.
