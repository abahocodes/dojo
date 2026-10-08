Given an integer array `arr` and an integer `target`, count the triples of
positions `i < j < k` with `arr[i] + arr[j] + arr[k] == target`.

Triples are counted by position, so equal values at different positions give
different triples. The count can be very large: return it modulo
`10^9 + 7`.

## Example 1

```
arr    = [1, 2, 3, 4, 5]
target = 9
output = 2    # 1 + 3 + 5 and 2 + 3 + 4
```

## Example 2

```
arr    = [2, 2, 2, 2]
target = 6
output = 4    # any 3 of the 4 positions
```

## Constraints

- `3 <= len(arr) <= 3000`
- `0 <= arr[i] <= 100`
- `0 <= target <= 300`
