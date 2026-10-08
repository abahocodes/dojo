You are given `head`, the first node of a singly linked list, and an integer
`k`.

Split the list into consecutive groups of `k` nodes, starting from the head,
and reverse the order of the nodes inside every complete group. If the last
group has fewer than `k` nodes, leave it in its original order. Return the head
of the modified list.

Rearrange the nodes by changing their `next` pointers; do not just swap the
values stored in them. Aim for O(1) extra memory.

## Example 1

```
head   = 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8
k = 3
output = 3 -> 2 -> 1 -> 6 -> 5 -> 4 -> 7 -> 8
```

The groups are `1 2 3`, `4 5 6` and the incomplete `7 8`, which stays as is.

## Example 2

```
head   = 9 -> 4 -> 0 -> 6
k = 2
output = 4 -> 9 -> 6 -> 0
```

## Constraints

- The list has `n` nodes, with `1 <= k <= n <= 5000`.
- `0 <= node.val <= 1000`
