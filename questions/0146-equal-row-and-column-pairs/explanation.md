# Approach: hash the rows, look up the columns

Two sequences are equal exactly when their keys are equal, so put every row
into a hash map (key: the row's values, value: how many rows have them).
Then build each column, look it up, and add the number of identical rows.

```python
def equal_pairs(grid):
    rows = {}
    for row in grid:
        key = tuple(row)
        rows[key] = rows.get(key, 0) + 1
    return sum(rows.get(tuple(col), 0) for col in zip(*grid))
```

In languages without hashable arrays, join the values into a string with a
separator, use a list type with value equality, or use an ordered map keyed
by the vector (C++ `map<vector<int>, int>`).

## Complexity

- Time: O(n^2): every cell is read twice and every key has length n (plus
  the hashing of `2n` keys of length `n`).
- Space: O(n^2) for the row keys.

## Pitfalls

- Joining numbers without a separator: rows `[1, 23]` and `[12, 3]` would
  both become `"123"`.
- Using a set instead of counts: duplicate rows must each be counted.
- Building column keys from `grid[c]` instead of `grid[r][c]` for all `r`.
