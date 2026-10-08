You are given an array `lists` of `k` linked lists, each sorted in ascending order. Merge all of them into a single sorted linked list and return its head.

An entry in `lists` may be `null`, meaning that particular list is empty. If every list is empty (or the array itself is empty), return `null`.

## Example 1

```
lists = [1 -> 4 -> 5, 1 -> 3 -> 4, 5 -> 6 -> 10]
output = 1 -> 1 -> 3 -> 4 -> 4 -> 5 -> 5 -> 6 -> 10
```

## Example 2

```
lists = []
output = null
```

## Constraints

- `0 <= len(lists) <= 10^4`
- Each list is sorted in non-decreasing order.
- The total number of nodes across all lists is at most `10^4`.
- `-10^4 <= node.val <= 10^4`

**Follow-up:** can you do better than merging the lists one at a time?
