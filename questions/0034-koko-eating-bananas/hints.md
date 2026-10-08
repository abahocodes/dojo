# Hints

## Hint 1
For a fixed speed `v`, how many hours does pile `p` take? Sum that over all
piles. Speed `max(piles)` always works, and speed `1` is the slowest possible.

## Hint 2
If speed `v` lets her finish in time, so does every faster speed. That
monotonic yes/no answer is what binary search needs.

## Hint 3
Binary search `v` in `[1, max(piles)]`. Pile `p` takes `ceil(p / v)` hours,
which is `(p + v - 1) // v` in integers. If the total is `<= h`, try smaller
(`hi = v`), otherwise `lo = v + 1`.
