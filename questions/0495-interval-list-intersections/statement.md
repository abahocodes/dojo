You are given two lists of closed intervals, `first` and `second`. Within each
list the intervals are pairwise disjoint (no shared points) and sorted by start.

Return the intersection of the two lists: every maximal stretch of numbers that
is covered by some interval of `first` **and** by some interval of `second`,
written as closed intervals `[lo, hi]` and sorted by `lo`. When two intervals
share only a single point `x`, the intersection contains `[x, x]`.

## Example 1

```
first  = [[0, 4], [7, 10], [13, 20]]
second = [[2, 8], [10, 15], [18, 25]]
output = [[2, 4], [7, 8], [10, 10], [13, 15], [18, 20]]
```

## Example 2

```
first  = [[1, 3], [6, 6]]
second = [[3, 5]]
output = [[3, 3]]
```

## Constraints

- `0 <= len(first), len(second) <= 1000`
- `0 <= start <= end <= 10^9` for every interval
- Each list is sorted by start, and its intervals share no point.
