A row of `n` cities is numbered `0` to `n - 1`. City `j` already hosts
`stations[j]` power stations, and every station has range `r`: a station in
city `j` supplies power to each city `i` with `|i - j| <= r`. The **power** of
a city is the number of stations that supply it.

You are allowed to build `k` additional stations, each in any city you like
(several may go in the same city). They have the same range `r`.

Return the largest value that the **minimum** power over all cities can reach
after the new stations are placed optimally.

## Example 1

```
stations = [1, 2, 4, 5, 0]
r        = 1
k        = 2
output   = 5
# Powers start as [3, 7, 11, 9, 5]. Building both stations in city 1
# gives [5, 9, 13, 9, 5], whose minimum is 5.
```

## Example 2

```
stations = [4, 4, 4, 4]
r        = 0
k        = 3
output   = 4
# Each station only covers its own city, and three stations cannot lift
# all four cities to 5.
```

## Constraints

- `1 <= n == len(stations) <= 10^5`
- `0 <= stations[i] <= 10^5`
- `0 <= r <= n - 1`
- `0 <= k <= 10^9`
- The answer can exceed `2^31 - 1`; return it as a 64-bit integer.
