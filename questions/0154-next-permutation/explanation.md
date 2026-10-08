# Approach: pivot, swap, reverse

1. Scan from the right for the first index `i` with `nums[i] < nums[i + 1]`.
   Everything after `i` is non-increasing, so no rearrangement of that
   suffix alone gives a larger array: position `i` must increase.
2. If no such `i` exists, the array is the largest arrangement. Reversing
   it gives the ascending order, which is the wrap-around answer.
3. Otherwise scan from the right for the first `j > i` with
   `nums[j] > nums[i]`. Because the suffix is non-increasing, this is the
   smallest value larger than `nums[i]` (and its rightmost copy). Swap them.
4. The suffix is still non-increasing after the swap. Reverse it so it
   becomes the smallest possible tail.

```python
def next_permutation(nums):
    n = len(nums)
    i = n - 2
    while i >= 0 and nums[i] >= nums[i + 1]:
        i -= 1
    if i >= 0:
        j = n - 1
        while nums[j] <= nums[i]:
            j -= 1
        nums[i], nums[j] = nums[j], nums[i]
    lo, hi = i + 1, n - 1
    while lo < hi:
        nums[lo], nums[hi] = nums[hi], nums[lo]
        lo += 1
        hi -= 1
    return nums
```

## Complexity

- Time: O(n): each of the three scans is linear.
- Space: O(1) extra.

## Pitfalls

- Using `>` instead of `>=` in the first scan (or `<` instead of `<=` in
  the second): with duplicates such as `[1, 5, 5]` the pivot or swap target
  comes out wrong.
- Sorting the suffix instead of reversing it: correct, but O(n log n) when
  a reversal suffices.
- Forgetting the wrap-around case, where `i` ends at `-1`.
