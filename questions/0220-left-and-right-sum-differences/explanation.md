# Approach: running prefix sum

Everything at index `i` splits into three parts: the left total, `nums[i]`
itself, and the right total. These add up to the sum of the whole array, so
once we know `total` and the running left total, the right total is just
`total - left - nums[i]`. One pass computes every answer.

```python
def left_right_difference(nums):
    total = sum(nums)
    left = 0
    result = []
    for x in nums:
        right = total - left - x
        result.append(abs(left - right))
        left += x
    return result
```

The largest possible total is `1000 * 10^5 = 10^8`, which fits a 32-bit int.

## Complexity

- Time: O(n): one pass for the total, one for the answers.
- Space: O(1) besides the output array.

## Pitfalls

- Including `nums[i]` in the left or right total. Both sides exclude it.
- Updating `left` before computing the answer for `i`, which shifts every
  answer by one element.
- Forgetting the absolute value: the left total is often the smaller one.
