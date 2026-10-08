Two non-negative integers are stored as linked lists, one decimal digit per
node, with the **least significant digit first**. For example, the number 352
is stored as `2 -> 5 -> 3`.

Given the heads `l1` and `l2` of two such lists, return the sum of the two
numbers as a linked list in the same reversed-digit format.

Neither input has extra leading zeros, except that the number zero itself is a
single node `0`. Your result must not have leading zeros either. The numbers
can be far too long to fit in a machine integer.

## Example 1

```
l1     = 4 -> 1 -> 7          # 714
l2     = 9 -> 8 -> 2          # 289
output = 3 -> 0 -> 0 -> 1     # 1003
```

## Example 2

```
l1     = 5 -> 6               # 65
l2     = 0                    # 0
output = 5 -> 6               # 65
```

## Constraints

- Each list has between `1` and `3000` nodes.
- `0 <= node.val <= 9`
- Neither list has leading zeros, except for the number `0` itself.
