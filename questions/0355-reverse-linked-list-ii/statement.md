You are given `head`, the first node of a singly linked list, and two integers
`left` and `right` with `left <= right`. Positions are numbered from `1`.

Reverse the run of nodes from position `left` through position `right`
(inclusive), leaving the nodes before and after it untouched. Return the head
of the resulting list.

Aim to do it in a single pass over the list.

## Example 1

```
head   = 3 -> 1 -> 4 -> 1 -> 5 -> 9
left   = 2
right  = 5
output = 3 -> 5 -> 1 -> 4 -> 1 -> 9
```

## Example 2

```
head   = 6 -> 2
left   = 1
right  = 2
output = 2 -> 6
```

## Constraints

- The list has between `1` and `500` nodes (call it `n`).
- `-500 <= node.val <= 500`
- `1 <= left <= right <= n`
