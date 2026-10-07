# Approach: bottom-up DP with two variables

Every route to step `n` ends with either a one-step move from `n - 1` or a
two-step move from `n - 2`. These two groups don't overlap, so

```
ways(n) = ways(n - 1) + ways(n - 2),   ways(0) = 1,   ways(1) = 1
```

This is the Fibonacci recurrence. Computing it top-down without memoisation
takes O(φⁿ) calls, which is far too slow for `n = 45`. Iterating upward and
keeping only the last two values is O(n) time and O(1) space:

```python
def climb_stairs(n):
    prev, curr = 1, 1  # ways to reach step 0 and step 1
    for _ in range(n - 1):
        prev, curr = curr, prev + curr
    return curr
```

A memoised recursion (`functools.cache`) is also fine and is often the
easiest way to get there from the recurrence.

## Complexity

- Time: O(n).
- Space: O(1) for the iterative version, O(n) for memoised recursion.

## Pitfalls

- Naive recursion with no cache times out well before `n = 45`.
- Off-by-one in the base cases: `ways(1) = 1` and `ways(2) = 2`. Treating
  `ways(0)` as `0` breaks the recurrence.
- In fixed-width languages the answer for `n = 45` (1,836,311,903) still fits a
  signed 32-bit integer, but only just.
