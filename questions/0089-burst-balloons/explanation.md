# Approach: interval DP on the last balloon popped

Add a virtual balloon worth `1` at each end: `v = [1] + nums + [1]`. These are
never popped; they just supply the "missing neighbour counts as 1" rule.

Let `best[i][j]` be the most points obtainable by popping **every balloon
strictly between** `i` and `j` while `i` and `j` themselves stay. Choose the
balloon `k` (with `i < k < j`) that is popped **last** in that range. Until
then `k` stands between the left part `(i, k)` and the right part `(k, j)`, so
the two parts are independent subproblems, and when `k` finally pops its
neighbours are `i` and `j`:

```
best[i][j] = max(best[i][k] + v[i] * v[k] * v[j] + best[k][j])  for i < k < j
best[i][i+1] = 0
```

Process intervals in order of increasing length so the smaller ones are ready.

```python
def max_coins(nums):
    v = [1] + nums + [1]
    n = len(v)
    best = [[0] * n for _ in range(n)]
    for length in range(2, n):
        for i in range(n - length):
            j = i + length
            edge = v[i] * v[j]
            best[i][j] = max(best[i][k] + edge * v[k] + best[k][j]
                             for k in range(i + 1, j))
    return best[0][n - 1]
```

## Complexity

- Time: O(n^3): O(n^2) intervals, each trying O(n) last balloons.
- Space: O(n^2).

## Pitfalls

- Choosing the **first** balloon to pop gives subproblems that still depend on
  each other; it leads to exponential search.
- Greedy rules (pop the smallest first, or the one with the biggest product)
  are wrong; for `[3, 1, 5, 8]` the best is 167 and no simple greedy finds it.
- Balloons worth `0` are legal. Don't drop them from the row: while a `0` is
  still standing it shields its neighbours from each other. The DP handles
  them with no special case.
- Iterate intervals by length (or `i` downwards, `j` upwards) so that
  `best[i][k]` and `best[k][j]` are computed before `best[i][j]`.
