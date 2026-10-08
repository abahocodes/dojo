There are `n` workers. Worker `i` has a skill rating `quality[i]` and will
only accept a job paying at least `wage[i]`. You must hire **exactly** `k`
of them, and the payroll for the group has to be fair:

1. every hired worker receives at least their minimum `wage[i]`, and
2. pay is proportional to quality: if one hired worker's quality is twice
   another's, their pay is also exactly twice the other's.

Pay amounts do not need to be whole numbers. Return the smallest possible
total pay for a group of `k` workers. Answers within `1e-6` (relative) of the
expected value are accepted.

## Example 1

```
quality = [4, 2, 5]
wage    = [20, 14, 30]
k       = 2
output  = 42.0
```

Hire workers 0 and 1. Worker 1 needs at least 14 for quality 2, i.e. 7 per
unit of quality, which also covers worker 0 (4 * 7 = 28 >= 20). Total
`28 + 14 = 42`. The groups containing worker 2 cost more: `54` with worker 0
(rate 6) and `49` with worker 1 (rate 7).

## Example 2

```
quality = [3, 1, 10, 10, 1]
wage    = [4, 8, 2, 2, 7]
k       = 3
output  = 30.666666666666668
```

Hire workers 0, 2 and 3. Worker 0 sets the rate at `4/3` per unit of quality,
and the total quality is `23`, so the total pay is `23 * 4/3 = 92/3`.

## Constraints

- `n == len(quality) == len(wage)`
- `1 <= k <= n <= 10^4`
- `1 <= quality[i], wage[i] <= 10^4`
