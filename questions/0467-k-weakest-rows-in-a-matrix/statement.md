A binary matrix `mat` describes a battlefield: `1` is a soldier and `0` is a
civilian. In every row, all soldiers stand to the **left** of all civilians
(each row is some 1s followed by some 0s).

Row `i` is **weaker** than row `j` when it has fewer soldiers, or when both
have the same number of soldiers and `i < j`.

Return the indices of the `k` weakest rows, ordered from the weakest to the
strongest.

## Example 1

```
mat    = [[1, 1, 0, 0],
          [1, 1, 1, 1],
          [1, 0, 0, 0],
          [1, 1, 0, 0],
          [0, 0, 0, 0]]
k      = 3
output = [4, 2, 0]
```

Soldier counts are `[2, 4, 1, 2, 0]`. Row 4 has none, row 2 has one, and rows
0 and 3 tie with two, so the lower index (0) is weaker.

## Example 2

```
mat    = [[1, 0],
          [1, 0],
          [1, 1]]
k      = 2
output = [0, 1]
```

## Constraints

- `2 <= len(mat) <= 100` (rows)
- `2 <= len(mat[i]) <= 100` (columns; all rows have the same length)
- `1 <= k <= len(mat)`
- `mat[i][j]` is `0` or `1`, and each row is all 1s followed by all 0s.
