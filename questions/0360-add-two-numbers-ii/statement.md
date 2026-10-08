Two non-negative integers are stored as singly linked lists `l1` and `l2`, one
decimal digit per node, with the **most significant digit first**. Neither
number has leading zeros, except the number `0` itself, which is the single
node `0`.

Return their sum as a linked list in the same format (most significant digit
first, no leading zeros).

## Example 1

```
l1     = 7 -> 2 -> 4 -> 3
l2     = 5 -> 6 -> 4
output = 7 -> 8 -> 0 -> 7
```

`7243 + 564 = 7807`.

## Example 2

```
l1     = 9 -> 9
l2     = 1
output = 1 -> 0 -> 0
```

## Constraints

- Each list has between `1` and `100` nodes.
- `0 <= node.val <= 9`
- Neither list has a leading zero unless it is exactly the number `0`.
