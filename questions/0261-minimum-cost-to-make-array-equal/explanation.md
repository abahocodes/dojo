# Approach: weighted median

For a target value `x`, the cost is `f(x) = sum(cost[i] * |nums[i] - x|)`.
Imagine `cost[i]` copies of `nums[i]` on a number line. Moving `x` one step
right increases the distance to every copy at or left of `x` and decreases
the distance to every copy right of it. So `f` keeps decreasing while less
than half of the total weight lies at or left of `x`, and stops decreasing
once at least half does. The first value where the prefix weight reaches half
of the total is a **weighted median**, and it minimizes `f`.

Sort the indices by `nums`, accumulate `cost` until `2 * prefix >= total`,
take that `nums` value as the target, and sum the cost.

```python
def min_cost(nums, cost):
    pairs = sorted(zip(nums, cost))
    total = sum(cost)
    acc = 0
    for value, weight in pairs:
        acc += weight
        if 2 * acc >= total:
            target = value
            break
    return sum(w * abs(v - target) for v, w in pairs)
```

An equivalent approach: sort, build prefix sums of `cost` and `cost * nums`,
and evaluate `f` at every `nums` value in `O(1)` each, taking the minimum.

## Complexity

- Time: `O(n log n)` for the sort; the rest is linear.
- Space: `O(n)` for the sorted pairs.

## Pitfalls

- Using the plain (unweighted) median. With `nums = [1, 2, 10]`,
  `cost = [1, 1, 100]` the middle element `2` costs `801`, while the weighted
  median `10` costs only `17`.
- Overflow: `cost[i] * |nums[i] - x|` can reach `10^12`, and the sum is far
  beyond 32 bits. Use `long` / `long long` / 64-bit `int`.
- Trying every target value in `[min(nums), max(nums)]` with a full `O(n)`
  scan each: that is up to `10^6 * 10^5` operations.
