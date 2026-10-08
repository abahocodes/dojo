You are given `head`, the first node of a singly linked list with `n` nodes.
Number the nodes from `0` to `n - 1`. The **middle** node is the one at index
`floor(n / 2)`.

Remove the middle node from the list and return the head of the resulting list.
If the list has only one node, the result is the empty list.

## Example 1

```
head   = 2 -> 8 -> 5 -> 1 -> 9
output = 2 -> 8 -> 1 -> 9
```

The list has 5 nodes, so the middle is index `2` (value `5`).

## Example 2

```
head   = 6 -> 3 -> 7 -> 4
output = 6 -> 3 -> 4
```

With 4 nodes the middle is index `2` (value `7`).

## Constraints

- The list has between `1` and `10^5` nodes.
- `1 <= node.val <= 10^5`
