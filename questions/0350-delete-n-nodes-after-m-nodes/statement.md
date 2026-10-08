You are given `head`, the first node of a singly linked list, and two positive
integers `m` and `n`.

Starting at the head, repeat the following until the end of the list is
reached:

1. keep the next `m` nodes (or all remaining nodes, if fewer than `m` are left);
2. delete the next `n` nodes (or all remaining nodes, if fewer than `n` are
   left).

Return the head of the modified list. The kept nodes stay in their original
order.

## Example 1

```
head   = 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10
m      = 3
n      = 2
output = 1 -> 2 -> 3 -> 6 -> 7 -> 8
```

Keep `1 2 3`, delete `4 5`, keep `6 7 8`, delete `9 10`.

## Example 2

```
head   = 5 -> 4 -> 3 -> 2 -> 1
m      = 1
n      = 1
output = 5 -> 3 -> 1
```

## Constraints

- The list has between `1` and `10^4` nodes.
- `1 <= node.val <= 10^6`
- `1 <= m, n <= 1000`
