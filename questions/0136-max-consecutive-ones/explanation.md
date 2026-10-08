# Approach: one pass with a running count

Keep `run`, the number of consecutive 1s ending at the current element, and
`best`, the largest `run` seen. A `1` extends the run, a `0` resets it. The
answer is `best` after the scan.

```python
def find_max_consecutive_ones(nums):
    best = run = 0
    for x in nums:
        run = run + 1 if x == 1 else 0
        best = max(best, run)
    return best
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Updating `best` only when a `0` is seen misses a run that reaches the end
  of the array. Update it on every `1` (or once more after the loop).
- An array of only `0`s has answer `0`.
