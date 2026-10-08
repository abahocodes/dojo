## Hint 1
Fix a target sum `T` first. For one pair `(a, b)`, how many moves does it need
to reach `T`? The answer is always 0, 1 or 2.

## Hint 2
A pair needs 0 moves when `T == a + b`. With one move you can change either
element, so one move reaches any `T` from `min(a, b) + 1` to
`max(a, b) + limit`. Every other `T` in `[2, 2 * limit]` costs 2 moves.

## Hint 3
Trying every `T` against every pair is too slow. Instead, each pair adds a
step function over `T`: record where its cost changes in a difference array
(+2 at 2, -1 at `lo + 1`, -1 at `a + b`, +1 at `a + b + 1`, +1 at
`hi + limit + 1`), then take a running sum over `T` and keep the minimum.
