# Approach: total minus running prefix

With `total = sum(nums)` and `left` the sum of `nums[0..i-1]`, the right-hand
side of index `i` sums to `total - left - nums[i]`. Scan from the left and
return the first index where the two sides match; add `nums[i]` to `left`
before moving on.

```python
def pivot_index(nums):
    total = sum(nums)
    left = 0
    for i, x in enumerate(nums):
        if left == total - left - x:
            return i
        left += x
    return -1
```

## Complexity

- Time: O(n): one pass for the total, one for the scan.
- Space: O(1).

## Pitfalls

- Including `nums[i]` in either side. The balance point itself is excluded.
- Forgetting the ends: index `0` has an empty left side, the last index an
  empty right side. Both can be balance points.
- Negative numbers break any "stop when the left side gets too big" shortcut;
  check every index.
- Returning the last match instead of the first.
