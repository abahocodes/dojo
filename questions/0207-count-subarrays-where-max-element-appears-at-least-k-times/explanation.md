# Approach: sliding window counting valid starts

Fix the right end `r`. Whether `nums[l..r]` holds at least `k` copies of `M`
is monotone in `l`: moving `l` left can only add copies. So the valid starts
for this `r` are exactly `0, 1, ..., left - 1`, where `left` is the first start
whose window has fewer than `k` copies.

As `r` grows, `left` never moves back, so both pointers sweep the array once.
After extending the window with `nums[r]`, shrink it from the left while it
still has at least `k` copies; the number of starts we skipped is `left`.

```python
def count_subarrays_max_k(nums, k):
    m = max(nums)
    count = 0
    left = 0
    total = 0
    for value in nums:
        if value == m:
            count += 1
        while count >= k:
            if nums[left] == m:
                count -= 1
            left += 1
        total += left
    return total
```

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Using the maximum of each subarray instead of the global maximum `M`.
- Overflow: up to about 5 * 10^9 subarrays qualify, which does not fit a
  32-bit integer.
- Counting subarrays with *exactly* `k` copies instead of *at least* `k`.
- `k` larger than the number of copies of `M`: the answer is simply `0`.
