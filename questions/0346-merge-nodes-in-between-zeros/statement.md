You are given `head`, the first node of a singly linked list of non-negative
integers. The list both starts and ends with a node holding `0`, and no two
`0` nodes are adjacent.

The `0` nodes split the list into groups of consecutive non-zero nodes. Replace
each group by a single node whose value is the sum of the group, remove every
`0` node, and return the head of the resulting list. The new nodes appear in
the same order as their groups.

## Example 1

```
head   = 0 -> 4 -> 1 -> 0 -> 6 -> 0 -> 2 -> 2 -> 2 -> 0
output = 5 -> 6 -> 6
```

## Example 2

```
head   = 0 -> 7 -> 0
output = 7
```

## Constraints

- The list has between `3` and `2 * 10^5` nodes.
- `0 <= node.val <= 1000`
- The first and last nodes hold `0`, and no two adjacent nodes both hold `0`.
