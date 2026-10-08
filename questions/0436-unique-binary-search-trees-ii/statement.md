Given an integer `n`, build every binary search tree whose nodes hold exactly
the values `1, 2, ..., n` (each value once). Two trees count as different when
their shapes differ; since the values are fixed, the shape determines where
every value sits.

Return the roots of all such trees. The list may be in **any order**, but it
must contain each distinct tree exactly once.

Each tree is shown below in level order, with `null` for a missing child.

## Example 1

```
n      = 3
output = [[1, null, 2, null, 3], [1, null, 3, 2], [2, 1, 3],
          [3, 1, null, null, 2], [3, 2, null, 1]]
```

## Example 2

```
n      = 1
output = [[1]]
```

## Constraints

- `1 <= n <= 8`
- The trees may be returned in any order.
