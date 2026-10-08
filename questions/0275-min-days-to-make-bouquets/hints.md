# Hints

## Hint 1
When is the answer `-1`? You need `m * k` distinct flowers, and once every
flower is open you can always cut the row into consecutive blocks of `k`.
Beware: `m * k` can reach `10^11`.

## Hint 2
For a fixed day `d`, can you count the bouquets available? Scan left to
right, tracking the length of the current run of open flowers
(`bloom_day[i] <= d`); every time the run reaches `k`, take a bouquet and
reset the run.

## Hint 3
More days never means fewer bouquets, so the feasible days form a suffix.
Binary search `d` between `min(bloom_day)` and `max(bloom_day)`: if day `mid`
yields at least `m` bouquets, the answer is `mid` or earlier
(`hi = mid`), otherwise later (`lo = mid + 1`).
