# Approach: fixed-size window with a frequency table

Slide a window of length `k` across the array. For each step add the entering
element and remove the leaving one, updating

- `window`: the sum of the current elements,
- `count[v]`: how many times `v` occurs in the window,
- `dup`: how many values occur at least twice.

A window has all-distinct elements exactly when `dup == 0`; among those keep
the largest sum.

```python
def maximum_subarray_sum_distinct(nums, k):
    count = {}
    dup = 0
    window = 0
    best = 0
    for i, value in enumerate(nums):
        window += value
        count[value] = count.get(value, 0) + 1
        if count[value] == 2:
            dup += 1
        if i >= k:
            old = nums[i - k]
            window -= old
            count[old] -= 1
            if count[old] == 1:
                dup -= 1
        if i >= k - 1 and dup == 0:
            best = max(best, window)
    return best
```

Because values are at most 10^5, the Java, C++ and Go solutions use an array
of counters instead of a hash map.

## Complexity

- Time: O(n).
- Space: O(min(n, V)) for the counters, where V is the value range.

## Pitfalls

- Overflow: a window can sum to 10^10, beyond 32-bit range.
- Re-checking distinctness of each window with a fresh set: O(n * k).
- Returning a negative sentinel or the best sum of an invalid window when no
  window qualifies; the answer must be `0` then.
- Updating `dup` only when a count goes from 1 to 2 but forgetting the
  matching decrement when it goes from 2 back to 1.
