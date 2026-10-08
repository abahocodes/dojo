# Hints

## Hint 1
Building `k` complete rows uses `1 + 2 + ... + k = k * (k + 1) / 2` coins.
You want the largest `k` for which that is at most `n`.

## Hint 2
`k * (k + 1) / 2` grows with `k`, so binary search on `k`. What upper bound
keeps the multiplication safe when `n` is close to `2^52`?

## Hint 3
`k * (k + 1) / 2 <= 2^52` forces `k < 94906266`. Search `k` in
`[0, min(n, 94906266)]`, rounding `mid` up: keep `mid` if
`mid * (mid + 1) / 2 <= n`, otherwise discard it and everything above.
