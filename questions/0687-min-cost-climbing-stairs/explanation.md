# Approach: dynamic programming over positions

Number the positions `0` to `n`, where `n` is the landing. Let `best[i]`
be the smallest cost of **standing** on position `i` (before paying for it).

- You may start on step `0` or step `1` for free: `best[0] = best[1] = 0`.
- You reach position `i` by paying for step `i - 1` and climbing one, or for
  step `i - 2` and climbing two:
  `best[i] = min(best[i - 1] + cost[i - 1], best[i - 2] + cost[i - 2])`.

The answer is `best[n]`. Since each value only looks two back, keep two
running variables instead of the whole table.

```python
def min_cost_climbing_stairs(cost):
    a, b = 0, 0              # best[i - 2], best[i - 1]
    for i in range(2, len(cost) + 1):
        a, b = b, min(b + cost[i - 1], a + cost[i - 2])
    return b
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) extra.

## Pitfalls

- The landing is position `n`, one past the last step. Returning `best[n - 1]`
  forgets that the last step must still be paid for (or skipped over).
- Both starting steps are free. Charging `cost[0]` or `cost[1]` up front
  double-counts, because you pay for a step when you leave it.
- Plain recursion without memoisation branches twice per step and is
  exponential; on 1,000 steps it never finishes.
