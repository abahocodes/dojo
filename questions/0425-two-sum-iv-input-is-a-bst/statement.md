You are given the `root` of a binary search tree with distinct values, and an
integer `k`. Return `true` if there are **two different nodes** whose values add
up to `k`, and `false` otherwise. A single node cannot be used twice.

## Example 1

```
root   = [5, 3, 6, 2, 4, null, 7]
k      = 9
output = true     # 3 + 6 (also 2 + 7, 4 + 5)
```

## Example 2

```
root   = [5, 3, 6, 2, 4, null, 7]
k      = 28
output = false
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `-10^4 <= node.val <= 10^4`, and all values are distinct.
- `-10^5 <= k <= 10^5`
