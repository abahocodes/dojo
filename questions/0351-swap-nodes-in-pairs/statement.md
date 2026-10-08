You are given `head`, the first node of a singly linked list (or `None` if the
list is empty).

Group the nodes into consecutive pairs — positions `0` and `1`, `2` and `3`,
and so on — and swap the two nodes of each pair. If the list has an odd length,
the last node has no partner and stays where it is.

You must rearrange the **nodes themselves** by changing `next` pointers; do not
just swap the stored values. Return the head of the rearranged list.

## Example 1

```
head   = 1 -> 2 -> 3 -> 4 -> 5 -> 6
output = 2 -> 1 -> 4 -> 3 -> 6 -> 5
```

## Example 2

```
head   = 7 -> 0 -> 9
output = 0 -> 7 -> 9
```

The third node has no partner, so it stays last.

## Constraints

- The list has between `0` and `1000` nodes.
- `0 <= node.val <= 100`
