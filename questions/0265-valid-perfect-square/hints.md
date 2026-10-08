# Hints

## Hint 1
If `num` is a perfect square, its root is some integer `r` between `1` and
`num`. Can you find it faster than trying every candidate?

## Hint 2
`r * r` increases with `r`, so you can binary search for an `r` with
`r * r == num`. Pick an upper bound that keeps `r * r` from overflowing.

## Hint 3
Search `r` in `[1, min(num, 2^26)]`. If `mid * mid` equals `num`, return
`true`; if it is smaller, search right; otherwise search left. If the window
empties, return `false`.
