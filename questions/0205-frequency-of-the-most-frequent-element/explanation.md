# Approach: sort, then a sliding window with a running sum

Since increments only go up, the final common value can always be taken to be
one of the original elements, `nums[r]`. The cheapest elements to lift to
`nums[r]` are the ones closest below it, so after sorting the best group for a
fixed `r` is a contiguous window `nums[l..r]`. Its cost is

```
nums[r] * (r - l + 1) - (nums[l] + ... + nums[r])
```

As `r` moves right the cost of a fixed `l` never decreases, so the left edge
only ever moves right as well: a classic two-pointer window.

```python
def max_frequency(nums, k):
    a = sorted(nums)
    left = 0
    window = 0
    best = 0
    for right, value in enumerate(a):
        window += value
        while value * (right - left + 1) - window > k:
            window -= a[left]
            left += 1
        best = max(best, right - left + 1)
    return best
```

## Complexity

- Time: O(n log n) for the sort; the window itself is O(n).
- Space: O(n) for the sorted copy (O(1) extra if sorting in place).

## Pitfalls

- Overflow: `nums[r] * window_length` reaches 10^10. Use 64-bit arithmetic in
  Java, C++ and Go.
- Forgetting to sort: the window argument only holds on sorted data.
- Trying to lower values. Only increments are allowed.
- Recomputing the window cost from scratch each time, which makes the
  solution quadratic.
