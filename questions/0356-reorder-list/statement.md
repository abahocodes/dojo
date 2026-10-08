You are given `head`, the first node of a singly linked list with nodes
`L0, L1, ..., L(n-1)` in order.

Rearrange the list so that it alternates between the front and the back:

```
L0 -> L(n-1) -> L1 -> L(n-2) -> L2 -> L(n-3) -> ...
```

Do this by changing `next` pointers only (do not rewrite node values), and
return the head of the rearranged list (which is still `L0`).

## Example 1

```
head   = 1 -> 2 -> 3 -> 4
output = 1 -> 4 -> 2 -> 3
```

## Example 2

```
head   = 10 -> 20 -> 30 -> 40 -> 50
output = 10 -> 50 -> 20 -> 40 -> 30
```

## Constraints

- The list has between `1` and `5 * 10^4` nodes.
- `1 <= node.val <= 1000`
