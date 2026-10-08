You are given two singly linked lists, `list1` and `list2`, and two integers
`a` and `b`.

Positions in `list1` are counted from `0` at its head. Remove the nodes of
`list1` at positions `a` through `b` (both inclusive) and put the whole of
`list2` in their place: the node at position `a - 1` should be followed by the
head of `list2`, and the tail of `list2` should be followed by the node that
was at position `b + 1`. Return the head of the resulting list.

## Example 1

```
list1  = 10 -> 11 -> 12 -> 13 -> 14 -> 15
a = 2, b = 3
list2  = 7 -> 8 -> 9
output = 10 -> 11 -> 7 -> 8 -> 9 -> 14 -> 15
```

## Example 2

```
list1  = 1 -> 2 -> 3
a = 1, b = 1
list2  = 40 -> 50
output = 1 -> 40 -> 50 -> 3
```

## Constraints

- `3 <= length of list1 <= 10^4`
- `1 <= a <= b < length of list1 - 1` (the first and last nodes of `list1`
  are never removed)
- `1 <= length of list2 <= 10^4`
- `-10^4 <= node.val <= 10^4`
