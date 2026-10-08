You are given an `n x n` matrix of integers in which every row is sorted in
ascending order from left to right and every column is sorted in ascending
order from top to bottom. Return the `k`-th smallest value in the whole matrix
(`k` is 1-indexed).

Duplicates count separately: the `k`-th smallest value is the value at index
`k - 1` of the sorted list of all `n * n` entries, not the `k`-th distinct
value.

## Example 1

```
matrix = [[1, 4, 9],
          [3, 8, 12],
          [6, 11, 14]]
k      = 5
output = 6   # sorted: 1, 3, 4, 6, 8, 9, 11, 12, 14
```

## Example 2

```
matrix = [[-2, 5],
          [5, 5]]
k      = 3
output = 5   # sorted: -2, 5, 5, 5
```

## Constraints

- `1 <= n <= 300`
- `-10^9 <= matrix[i][j] <= 10^9`
- each row and each column is sorted in non-decreasing order
- `1 <= k <= n^2`
- Aim for better than sorting all `n^2` values: binary search on the value
  range runs in O(n log(max - min)) time and O(1) extra space.
