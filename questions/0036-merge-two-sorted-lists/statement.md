You are given the heads of two singly linked lists, `list1` and `list2`. Each
list is already sorted in non-decreasing order, and either one may be empty.

Combine them into a single sorted linked list by re-linking the existing nodes,
and return the head of the combined list.

## Example 1

```
list1  = 2 -> 5 -> 8
list2  = 1 -> 5 -> 6 -> 10
output = 1 -> 2 -> 5 -> 5 -> 6 -> 8 -> 10
```

## Example 2

```
list1  = (empty)
list2  = 0 -> 3
output = 0 -> 3
```

## Constraints

- Each list has between `0` and `3000` nodes.
- `-10000 <= node.val <= 10000`
- Both lists are sorted in non-decreasing order.
