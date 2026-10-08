# Approach: hash set, count only from run starts

Put all values in a set. A value `x` starts a run exactly when `x - 1` is
missing. From each start, walk upward through `x + 1, x + 2, ...` while the
values are present and record the longest length found.

```python
def longest_consecutive_sequence(nums):
    values = set(nums)
    best = 0
    for x in values:
        if x - 1 not in values:
            length = 1
            while x + length in values:
                length += 1
            best = max(best, length)
    return best
```

## Complexity

- Time: O(n). The outer loop is O(n), and every value is stepped over by
  exactly one inner walk (the one starting at the beginning of its run), so
  the inner loops total O(n) as well.
- Space: O(n) for the set.

## Pitfalls

- Starting a walk from **every** value is O(n²) on input like
  `[1, 2, ..., n]`. The "is `x - 1` missing?" check is the whole trick.
- Iterate over the set, not the original list: with many duplicates of a run
  start, iterating the list repeats the same walk again and again.
- Return `0` for an empty list, not `1`.
- Sorting works too (O(n log n)), but remember to skip equal neighbours
  without resetting the current run.
