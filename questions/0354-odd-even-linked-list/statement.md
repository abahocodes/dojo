You are given `head`, the first node of a singly linked list (or `None` if the
list is empty). Number the nodes by position starting from `1`.

Reorder the list so that all nodes at **odd** positions (1st, 3rd, 5th, ...)
come first, followed by all nodes at **even** positions (2nd, 4th, ...). Within
each group, keep the nodes in their original relative order. Positions refer
to where a node sits in the list, not to its value.

Do it by relinking the nodes, using only O(1) extra space, and return the head
of the reordered list.

## Example 1

```
head   = 8 -> 3 -> 5 -> 0 -> 7
output = 8 -> 5 -> 7 -> 3 -> 0
```

Odd positions hold `8, 5, 7`; even positions hold `3, 0`.

## Example 2

```
head   = -2 -> 4 -> 4 -> 9 -> 1 -> 6
output = -2 -> 4 -> 1 -> 4 -> 9 -> 6
```

## Constraints

- The list has between `0` and `10^4` nodes.
- `-10^6 <= node.val <= 10^6`
