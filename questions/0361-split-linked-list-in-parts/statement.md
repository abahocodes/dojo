You are given `head`, the first node of a singly linked list with `n` nodes
(possibly empty), and an integer `k`.

Cut the list into exactly `k` consecutive parts so that:

- the parts, read in order, contain the original nodes in their original order;
- any two parts differ in length by at most one;
- an earlier part is never shorter than a later part.

If `n < k`, some trailing parts are empty lists. Return the `k` parts in order
(each one as the head of its own list, or an empty list).

## Example 1

```
head   = 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8, k = 3
output = [1 -> 2 -> 3, 4 -> 5 -> 6, 7 -> 8]
```

As arrays: `[[1, 2, 3], [4, 5, 6], [7, 8]]`.

## Example 2

```
head   = 4 -> 9, k = 4
output = [4, 9, (empty), (empty)]
```

As arrays: `[[4], [9], [], []]`.

## Constraints

- The list has between `0` and `1000` nodes.
- `0 <= node.val <= 1000`
- `1 <= k <= 50`
