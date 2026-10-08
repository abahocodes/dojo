Gas stations stand on a straight road at the integer positions in `stations`,
listed in strictly increasing order. You must build exactly `k` more stations.
A new station can go at any real position, including non-integer ones.

After building, look at the distances between each pair of neighbouring
stations and take the largest. Return the smallest value this largest distance
can have. Answers within `10^-6` of the true value (relative to its size when
it exceeds 1) are accepted.

## Example 1

```
stations = [0, 2, 4, 6, 8, 10, 12, 14, 16, 30]
k        = 3
output   = 3.5   # put all three stations inside the 14-long gap: 16, 19.5, 23, 26.5, 30
```

## Example 2

```
stations = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
k        = 9
output   = 0.5   # one new station in the middle of every gap
```

## Constraints

- `10 <= len(stations) <= 2000`
- `0 <= stations[i] <= 10^8`, strictly increasing
- `1 <= k <= 10^6`
