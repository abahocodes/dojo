You are given `head`, the first node of a singly linked list.

Delete every node for which some node **later** in the list (anywhere to its
right, not only the next one) has a **strictly greater** value. Return the head
of the list that remains. The surviving nodes keep their original order, so
their values form a non-increasing sequence.

## Example 1

```
head   = 4 -> 11 -> 3 -> 9 -> 2 -> 7
output = 11 -> 9 -> 7
```

`4` is beaten by `11`, `3` by `9` and `2` by `7`; nothing to the right of
`11`, `9` or `7` is larger.

## Example 2

```
head   = 6 -> 6 -> 2 -> 6
output = 6 -> 6 -> 6
```

Equal values do not remove each other.

## Constraints

- The list has between `1` and `10^5` nodes.
- `1 <= node.val <= 10^5`
