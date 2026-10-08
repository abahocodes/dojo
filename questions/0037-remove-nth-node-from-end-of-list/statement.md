You are given `head`, the first node of a non-empty singly linked list, and an
integer `n`. Delete the node that sits `n` positions from the **end** of the
list (`n = 1` means the last node) and return the head of the resulting list.

`n` is always between `1` and the length of the list, so the node to delete
always exists. If the list had a single node, the result is empty.

## Example 1

```
head   = 7 -> 3 -> 9 -> 4 -> 1,  n = 2
output = 7 -> 3 -> 9 -> 1        # 4 was second from the end
```

## Example 2

```
head   = 6 -> 2,  n = 2
output = 2                       # the head itself is removed
```

## Constraints

- The list has between `1` and `5000` nodes.
- `0 <= node.val <= 1000`
- `1 <= n <= length of the list`

**Follow-up:** can you do it in a single pass over the list?
