You are given `head`, the first node of a singly linked list with `n` nodes, and
an integer `k`.

Exchange the **values** of two nodes: the `k`-th node counting from the front
and the `k`-th node counting from the back (both counts start at 1). If they
are the same node, the list is unchanged. Return the head of the list.

## Example 1

```
head   = 1 -> 2 -> 3 -> 4 -> 5, k = 2
output = 1 -> 4 -> 3 -> 2 -> 5
```

## Example 2

```
head   = 8 -> 0 -> 6 -> 3, k = 4
output = 3 -> 0 -> 6 -> 8
```

The 4th node from the front is the last node, and the 4th from the back is the
first node.

## Constraints

- `1 <= k <= n <= 10^5`
- `0 <= node.val <= 100`
