You are given `head`, the first node of a non-empty singly linked list.

Sort the list into non-decreasing order using **insertion sort**: build a
sorted list starting from nothing, then detach the input's nodes one at a time,
front to back, and splice each one into its correct position in the sorted
list. Return the head of the sorted list.

## Example 1

```
head   = 4 -> 2 -> 1 -> 3
output = 1 -> 2 -> 3 -> 4
```

## Example 2

```
head   = -1 -> 5 -> 3 -> 4 -> 0
output = -1 -> 0 -> 3 -> 4 -> 5
```

## Constraints

- The list has between `1` and `2000` nodes.
- `-5000 <= node.val <= 5000`
