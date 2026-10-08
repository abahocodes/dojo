# Approach: fix two rows, count subarrays

A submatrix is fixed by its row span `[top, bottom]` and its column span. For
a fixed row span, let `col[c]` be the sum of column `c` over those rows. The
submatrices with that row span are exactly the subarrays of `col`, and their
sums are subarray sums of `col`.

Counting subarrays of `col` that sum to `target` is the classic prefix-sum
problem: walk the array keeping a running sum `s` and a map of how many
earlier prefixes had each sum. Every earlier prefix equal to `s - target`
ends a qualifying subarray here.

For each `top`, extend `bottom` downward one row at a time and add that row
into `col`, so each row span costs O(cols).

```python
def num_submatrix_sum_target(matrix, target):
    rows, cols = len(matrix), len(matrix[0])
    count = 0
    for top in range(rows):
        col = [0] * cols
        for bottom in range(top, rows):
            row = matrix[bottom]
            for c in range(cols):
                col[c] += row[c]
            seen = {0: 1}
            s = 0
            for v in col:
                s += v
                count += seen.get(s - target, 0)
                seen[s] = seen.get(s, 0) + 1
    return count
```

If there are many more rows than columns, transpose first so that the
squared dimension is the smaller one.

## Complexity

- Time: O(rows^2 * cols).
- Space: O(cols) for the column sums and the map.

## Pitfalls

- Enumerating all four corners with a 2D prefix sum is O(rows^2 * cols^2),
  10^8 for a 100 x 100 grid: too slow.
- Forgetting `seen = {0: 1}` for the empty prefix, which misses submatrices
  starting at column `0`.
- Recording the current prefix sum before checking `s - target`. With
  `target = 0` that counts empty submatrices.
- Values and target can be negative, so a sliding window does not work.
