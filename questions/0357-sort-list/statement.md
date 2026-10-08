You are given `head`, the first node of a singly linked list (or `None` if the
list is empty).

Sort the nodes into non-decreasing order of value and return the head of the
sorted list. Rearrange the existing nodes; aim for `O(n log n)` time without
first copying the values into an array.

## Example 1

```
head   = 5 -> 1 -> 4 -> 2
output = 1 -> 2 -> 4 -> 5
```

## Example 2

```
head   = 3 -> -7 -> 3 -> 0 -> -2
output = -7 -> -2 -> 0 -> 3 -> 3
```

## Constraints

- The list has between `0` and `5 * 10^4` nodes.
- `-10^5 <= node.val <= 10^5`
