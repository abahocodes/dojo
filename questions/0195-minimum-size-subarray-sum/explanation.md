# Approach: variable-size sliding window

Because every number is positive, the sum of `nums[left..right]` grows as
`right` increases and shrinks as `left` increases. For each `right`, the best
window ending there starts at the largest `left` that still keeps the sum at
least `target`, and that `left` never moves backward as `right` advances. So
two pointers suffice:

```python
def min_sub_array_len(target, nums):
    best = len(nums) + 1
    window = 0
    left = 0
    for right, x in enumerate(nums):
        window += x
        while window >= target:
            best = min(best, right - left + 1)
            window -= nums[left]
            left += 1
    return 0 if best == len(nums) + 1 else best
```

The window sum never exceeds `target + 10^4 - 1 < 2^31`, because the inner loop
shrinks it as soon as it reaches `target`, so a 32-bit int is enough.

An alternative is to build prefix sums and, for each start, binary search the
first end whose prefix reaches `prefix[start] + target`: O(n log n).

## Complexity

- Time: O(n), each index enters and leaves the window once.
- Space: O(1).

## Pitfalls

- Using `>` instead of `>=`: a sum exactly equal to `target` qualifies.
- Shrinking with `if` instead of `while`: after adding a large number you may
  be able to drop several elements on the left.
- Returning the sentinel (`n + 1` or infinity) instead of `0` when nothing
  reaches `target`.
- This technique relies on positivity; it fails if negatives were allowed.
