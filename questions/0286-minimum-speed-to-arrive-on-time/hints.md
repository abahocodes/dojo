# Hints

## Hint 1
For a fixed speed `s`, the travel time is easy to compute: every train but the
last takes `ceil(dist[i] / s)` hours, and the last takes `dist[n-1] / s`.

## Hint 2
A faster speed never makes you later, so the set of good speeds is
`[answer, infinity)`. Binary search over `[1, 10^7]`.

## Hint 3
Avoid floating-point comparisons: let `H = round(hour * 100)`. With `S` the sum
of the rounded-up times, speed `s` works when `100 * S <= H` and
`100 * dist[n-1] <= (H - 100 * S) * s`. If the check fails even at `10^7`,
return `-1`.
