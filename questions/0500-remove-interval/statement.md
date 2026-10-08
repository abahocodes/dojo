A set of real numbers is described by a sorted list `intervals` of disjoint
half-open intervals: `[a, b)` contains every real `x` with `a <= x < b`.

Remove from this set every number in the half-open interval
`to_be_removed = [lo, hi)`. Return what is left in the same form: a list of
disjoint, non-empty half-open intervals sorted by start.

## Example 1

```
intervals     = [[0, 2], [3, 4], [5, 7]]
to_be_removed = [1, 6]
output        = [[0, 1], [6, 7]]
```

## Example 2

```
intervals     = [[-5, 5]]
to_be_removed = [-1, 2]
output        = [[-5, -1], [2, 5]]
```

## Constraints

- `1 <= len(intervals) <= 10^4`
- `-10^9 <= a < b <= 10^9` for every interval, and `-10^9 <= lo < hi <= 10^9`
- `intervals` is sorted, and consecutive intervals do not touch: the end of
  each is strictly smaller than the start of the next.
