You are given `head`, the first node of a singly linked list (or `None` if the
list is empty) whose values are sorted in non-decreasing order.

Remove nodes so that every distinct value appears exactly once - keep the first
node of each run of equal values and unlink the rest. Return the head of the
resulting list, which is still sorted.

## Example 1

```
head   = 2 -> 2 -> 5 -> 8 -> 8 -> 8 -> 9
output = 2 -> 5 -> 8 -> 9
```

## Example 2

```
head   = -3 -> 0 -> 4
output = -3 -> 0 -> 4    # already distinct
```

## Constraints

- The list has between `0` and `10^4` nodes.
- `-100 <= node.val <= 100`
- The values are sorted in non-decreasing order.
