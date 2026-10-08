You are given `head`, the first node of a non-empty singly linked list of
positive integers.

For every node, look further along the list for the **first** node whose value
is strictly larger than its own. Return a list, in the same order as the nodes,
holding that larger value for each node, or `0` if no later node is larger.

## Example 1

```
head   = 3 -> 8 -> 5 -> 6 -> 10 -> 2
output = [8, 10, 6, 10, 0, 0]
```

## Example 2

```
head   = 4 -> 4 -> 4
output = [0, 0, 0]      # equal values do not count as larger
```

## Constraints

- The list has between `1` and `10^4` nodes.
- `1 <= node.val <= 10^9`
