# Approach: sort, two nested anchors, two pointers

Sort `nums`. Choose the first value at index `i` and the second at index
`j > i`. The other two values are found in `nums[j+1:]` with two pointers
`lo` and `hi` closing in from both ends:

- if the four-value sum is below `target`, advance `lo`;
- if it is above, retreat `hi`;
- if it equals `target`, record the group and move `lo` and `hi` past all
  copies of their current values.

To avoid reporting the same group twice, skip an `i` whose value equals
`nums[i-1]`, and a `j` (with `j > i + 1`) whose value equals `nums[j-1]`.
Because the array is sorted, each distinct group is then produced exactly
once, already in non-decreasing order.

```python
def four_sum(nums, target):
    nums = sorted(nums)
    n = len(nums)
    res = []
    for i in range(n - 3):
        if i > 0 and nums[i] == nums[i - 1]:
            continue
        for j in range(i + 1, n - 2):
            if j > i + 1 and nums[j] == nums[j - 1]:
                continue
            lo, hi = j + 1, n - 1
            while lo < hi:
                s = nums[i] + nums[j] + nums[lo] + nums[hi]
                if s < target:
                    lo += 1
                elif s > target:
                    hi -= 1
                else:
                    res.append([nums[i], nums[j], nums[lo], nums[hi]])
                    lo += 1
                    while lo < hi and nums[lo] == nums[lo - 1]:
                        lo += 1
                    hi -= 1
                    while lo < hi and nums[hi] == nums[hi + 1]:
                        hi -= 1
    return res
```

## Complexity

- Time: O(n^3) after sorting: O(n^2) anchor pairs, O(n) pointer sweep each.
- Space: O(n) for the sorted copy, plus the output.

## Pitfalls

- 32-bit overflow: four values near 10^9 sum to about 4 * 10^9. Java, C++ and
  Go solutions must add in `long` / `long long` / `int64`.
- Deduplicating with a set of tuples works but hides the cost; skipping
  equal neighbours is cleaner and does not need extra memory.
- Skipping `j` when `nums[j] == nums[j-1]` even for `j == i + 1` loses groups
  such as `[2, 2, 2, 2]`.
- Arrays shorter than four elements: the answer is simply empty.
