# Approach: reduce to counting subsets with a given sum

Let `P` be the sum of the numbers that get a `+` and `N` the sum of the ones
that get a `-`. Every assignment satisfies

```
P - N = target
P + N = total
=> P = (total + target) / 2
```

So each valid assignment corresponds to exactly one subset (the `+` numbers)
summing to `goal = (total + target) / 2`, and vice versa. If `total + target`
is odd or negative, or `|target| > total`, there is no such subset.

Counting subsets with a fixed sum is a 0/1 knapsack: `ways[s]` is the number
of subsets of the numbers processed so far that sum to `s`. Sweep `s`
downwards so that each number is used at most once.

```python
def find_target_sum_ways(nums, target):
    total = sum(nums)
    if abs(target) > total or (total + target) % 2:
        return 0
    goal = (total + target) // 2
    ways = [1] + [0] * goal
    for x in nums:
        for s in range(goal, x - 1, -1):
            ways[s] += ways[s - x]
    return ways[goal]
```

Zeros need no special case: for `x = 0` the loop runs `s` down to `0` and
doubles every count, matching the fact that `+0` and `-0` are both allowed.

**Alternative:** a dictionary from running total to count, updated number by
number (`new[t + x] += c`, `new[t - x] += c`). It's the same idea without the
algebra, and costs O(n × total).

## Complexity

- Time: O(n × goal), at most 20 × 1,000.
- Space: O(goal).

## Pitfalls

- Skipping the parity / range check gives garbage (or a negative index) when
  `(total + target)` is odd or `target < -total`.
- Treating `+0` and `-0` as one assignment undercounts: each zero doubles the
  answer.
- Sweeping `s` upwards reuses the same number, counting assignments that don't
  exist.
- Plain recursion is 2^20 ≈ 10^6 calls: fine here, but it doesn't scale and the
  DP is just as short.
