An array `arr` is shaped like a **mountain**: starting from index `0` its
values strictly increase until they reach a single highest element, the
**peak**, and from there they strictly decrease until the last index. The
peak is never the first or the last element.

Return the index of the peak. Your solution should run in `O(log n)` time.

## Example 1

```
arr    = [1, 4, 9, 7, 2]
output = 2       # arr[2] = 9 is the top
```

## Example 2

```
arr    = [0, 10, 5]
output = 1
```

## Constraints

- `3 <= len(arr) <= 10^5`
- `0 <= arr[i] <= 10^6`
- `arr` is a mountain as described: there is an index `p` with
  `0 < p < len(arr) - 1`, `arr[0] < arr[1] < ... < arr[p]` and
  `arr[p] > arr[p + 1] > ... > arr[len(arr) - 1]`
