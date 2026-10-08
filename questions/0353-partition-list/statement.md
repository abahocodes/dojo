You are given `head`, the first node of a singly linked list (or `None` if the
list is empty), and an integer `x`.

Rearrange the list so that every node whose value is **less than** `x` comes
before every node whose value is **greater than or equal to** `x`. Within each
of these two groups, the nodes must keep the relative order they had in the
original list. Return the head of the rearranged list.

## Example 1

```
head   = 4 -> 1 -> 6 -> 2 -> 5 -> 2
x      = 4
output = 1 -> 2 -> 2 -> 4 -> 6 -> 5
```

The nodes below `4` are `1, 2, 2` (in original order), followed by `4, 6, 5`.

## Example 2

```
head   = 3 -> -1
x      = 0
output = -1 -> 3
```

## Constraints

- The list has between `0` and `200` nodes.
- `-200 <= node.val <= 200`
- `-200 <= x <= 200`
