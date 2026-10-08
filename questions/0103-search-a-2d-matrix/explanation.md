# Approach: binary search over the flattened grid

Because each row is sorted and every row starts above the previous row's end,
reading the grid row by row gives a single strictly increasing sequence of
`rows * cols` values. Binary search that sequence directly, translating each index
`i` to `matrix[i // cols][i % cols]`.

```python
def search_matrix(matrix, target):
    rows, cols = len(matrix), len(matrix[0])
    lo, hi = 0, rows * cols - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        value = matrix[mid // cols][mid % cols]
        if value == target:
            return True
        if value < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return False
```

## Alternative: two binary searches

First binary search the first column to find the last row whose first value is at
most `target`, then binary search inside that row. Same complexity, a little more
code.

## Complexity

- Time: O(log(rows * cols)).
- Space: O(1).

## Pitfalls

- The index-to-cell mapping divides by the number of **columns**, not rows — mixing
  them up only shows on non-square grids.
- Scanning from the top-right corner (step left or down) is O(rows + cols); it
  works, but it ignores the stronger second guarantee.
- Targets smaller than the first value or larger than the last must return `false`
  without indexing out of range.
