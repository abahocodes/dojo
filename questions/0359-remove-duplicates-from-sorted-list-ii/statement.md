You are given `head`, the first node of a singly linked list whose values are
in non-decreasing order (the list may be empty).

Remove **every** node whose value occurs more than once in the list, so that
only the values that appear exactly once remain. Return the head of the
resulting list, which is still sorted (it may be empty).

## Example 1

```
head   = 1 -> 2 -> 2 -> 3 -> 4 -> 4 -> 4 -> 5
output = 1 -> 3 -> 5
```

## Example 2

```
head   = 7 -> 7 -> 8 -> 9 -> 9
output = 8
```

## Constraints

- The list has between `0` and `10^4` nodes.
- `-100 <= node.val <= 100`
- The values are sorted in non-decreasing order.
