# Hints

## Hint 1
Instead of searching for the position of the answer, search for its value.
For a candidate value `v`, how many entries of the matrix are `<= v`?

## Hint 2
Counting entries `<= v` takes only O(n): start at the bottom-left corner. If
the entry is `<= v`, the whole column above it is too, so add `row + 1` and
step right; otherwise step up.

## Hint 3
The count is non-decreasing in `v`. Binary search `v` between `matrix[0][0]`
and `matrix[n-1][n-1]` for the smallest value whose count is at least `k`.
That value is always present in the matrix.
