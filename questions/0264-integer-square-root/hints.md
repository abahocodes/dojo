# Hints

## Hint 1
The function `r -> r * r` is increasing for `r >= 0`. That means the values
of `r` with `r * r <= x` form a prefix `0, 1, ..., answer`.

## Hint 2
Binary search for the last `r` in that prefix. What is a safe upper bound for
the search? Remember `x` can be as large as `2^52`.

## Hint 3
Search `r` in `[0, min(x, 2^26)]`. With `lo` always valid, pick
`mid = (lo + hi + 1) / 2`: if `mid * mid <= x` set `lo = mid`, else
`hi = mid - 1`. Stop when `lo == hi`. Rounding `mid` up stops the loop from
getting stuck.
