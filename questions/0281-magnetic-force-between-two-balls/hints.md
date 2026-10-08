# Hints

## Hint 1
Flip the question around: given a distance `g`, can you place all `m` balls
so that every pair is at least `g` apart? Sort the positions first.

## Hint 2
To test a distance `g`, walk left to right and put a ball in the first basket,
then in the next basket that is at least `g` beyond the previous ball, and so
on. Taking the earliest possible basket never hurts.

## Hint 3
If distance `g` is achievable, so is every smaller distance. Binary search for
the largest achievable `g` in `[1, (max - min) / (m - 1)]`, rounding the
midpoint up because the search keeps `lo` on success.
