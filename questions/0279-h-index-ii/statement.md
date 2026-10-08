A researcher has published `n` papers. You are given `citations`, where
`citations[i]` is the number of times paper `i` has been cited, and the array
is sorted in non-decreasing order.

The researcher's h-index is the largest integer `h` such that at least `h` of
the papers have each been cited at least `h` times. (`h = 0` always qualifies,
so the h-index is well defined.)

Return the h-index. Your algorithm must run in O(log n) time.

## Example 1

```
citations = [0, 2, 3, 5, 8]
output    = 3   # the papers with 3, 5 and 8 citations; there are not 4 papers with 4+
```

## Example 2

```
citations = [0, 0, 0]
output    = 0
```

## Constraints

- `1 <= len(citations) <= 10^5`
- `0 <= citations[i] <= 1000`
- `citations` is sorted in non-decreasing order
