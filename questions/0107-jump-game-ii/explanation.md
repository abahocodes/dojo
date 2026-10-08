# Approach: greedy BFS over ranges

Breadth-first search finds shortest paths in unweighted graphs, and here every
BFS level is a contiguous range of indices. Level 0 is `[0, 0]`; level `j + 1`
runs from just past level `j` to the furthest `i + nums[i]` over level `j`.

So scan left to right with `end` marking where the current level stops and
`furthest` tracking how far the next level reaches. Each time the scan arrives at
`end`, one more jump is needed and the next level ends at `furthest`. We loop only
up to `n - 2`: once we stand on the last index no further jump is needed.

```python
def jump(nums):
    jumps = 0
    end = 0
    furthest = 0
    for i in range(len(nums) - 1):
        furthest = max(furthest, i + nums[i])
        if i == end:
            jumps += 1
            end = furthest
    return jumps
```

## Complexity

- Time: O(n) — one pass.
- Space: O(1).

## Pitfalls

- Looping over the last index adds a spurious jump whenever a level ends exactly
  there (e.g. `[1, 1]` would return 2).
- DP over every jump length (`dp[i + d] = min(dp[i + d], dp[i] + 1)`) is correct
  but O(n · max(nums)).
- Greedily jumping to the index with the largest `nums` value is not the same
  thing — choose the jump whose *landing point plus its value* reaches furthest.
- A single-element list needs 0 jumps.
