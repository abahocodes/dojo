# Approach: exactly k = at most k minus at most k - 1

Counting subarrays with **at most** `k` distinct values is a standard sliding
window: for each right end `r`, the valid starts form a range `left..r`, and
`left` only moves forward as `r` grows. So `at_most(k)` takes one pass.

Every subarray with exactly `k` distinct values is counted by `at_most(k)` but
not by `at_most(k - 1)`, and every other subarray counted by `at_most(k)` is
counted by both. Hence

```
exactly(k) = at_most(k) - at_most(k - 1)
```

```python
def subarrays_with_k_distinct(nums, k):
    def at_most(limit):
        count = [0] * (len(nums) + 1)
        distinct = 0
        left = 0
        total = 0
        for right, value in enumerate(nums):
            if count[value] == 0:
                distinct += 1
            count[value] += 1
            while distinct > limit:
                old = nums[left]
                count[old] -= 1
                if count[old] == 0:
                    distinct -= 1
                left += 1
            total += right - left + 1
        return total

    return at_most(k) - at_most(k - 1)
```

Values lie in `1..n`, so a plain array of size `n + 1` replaces a hash map.

## Complexity

- Time: O(n), two linear passes.
- Space: O(n) for the counters.

## Pitfalls

- A single window for "exactly k": once the window has `k` distinct values,
  several different starts may be valid and a single left pointer misses them.
- Enumerating all subarrays: O(n^2) is too slow at the upper bound.
- `at_most(0)` must return `0` (the window can never be non-empty).
- The count reaches about 2 * 10^8 at most, which still fits a 32-bit integer.
