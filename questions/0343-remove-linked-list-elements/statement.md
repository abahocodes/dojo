You are given `head`, the first node of a singly linked list (or `None` if the
list is empty), and an integer `val`.

Unlink every node whose value equals `val` and return the head of what is left.
The surviving nodes must stay in their original relative order. If every node
is removed (or the list was empty), return an empty list.

## Example 1

```
head   = 6 -> 2 -> 6 -> 3 -> 4 -> 6
val    = 6
output = 2 -> 3 -> 4
```

## Example 2

```
head   = 1 -> 5 -> 9
val    = 7
output = 1 -> 5 -> 9    # nothing matches
```

## Constraints

- The list has between `0` and `10^4` nodes.
- `0 <= node.val <= 50`
- `0 <= val <= 50`
