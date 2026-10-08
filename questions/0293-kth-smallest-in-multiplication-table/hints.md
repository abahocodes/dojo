# Hints

## Hint 1
Instead of producing the `k`-th value directly, ask: for a candidate value `x`,
how many cells hold a value `<= x`?

## Hint 2
Row `i` holds `i, 2i, ..., n*i`, so it has `min(x // i, n)` cells that are at
most `x`. Summing over rows answers the question in O(m).

## Hint 3
That count only grows with `x`. Binary search for the smallest `x` in
`[1, m*n]` whose count is at least `k`. That `x` is always a value in the table.
