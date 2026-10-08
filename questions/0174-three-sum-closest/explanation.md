# Approach: sort, then two pointers per anchor

Sort `nums`. For each anchor index `i`, search the suffix `nums[i+1:]` for a
pair with `lo` at its left end and `hi` at its right end. With
`s = nums[i] + nums[lo] + nums[hi]`:

- record `s` if it is closer to `target` than the best so far;
- if `s < target`, every pair using `lo` with a smaller `hi` is even smaller,
  so `lo` is useless: advance it;
- if `s > target`, symmetrically retreat `hi`;
- if `s == target`, nothing can be closer: return it.

Each anchor costs O(n), so the whole search is O(n^2).

```python
def three_sum_closest(nums, target):
    nums = sorted(nums)
    n = len(nums)
    best = nums[0] + nums[1] + nums[2]
    for i in range(n - 2):
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = nums[i] + nums[lo] + nums[hi]
            if abs(s - target) < abs(best - target):
                best = s
            if s < target:
                lo += 1
            elif s > target:
                hi -= 1
            else:
                return s
    return best
```

## Complexity

- Time: O(n^2) after the O(n log n) sort.
- Space: O(n) for the sorted copy (O(1) extra if you sort in place).

## Pitfalls

- Returning the difference instead of the sum.
- Starting `best` at 0 or at a huge sentinel and then computing
  `abs(best - target)` with overflow; seeding it with a real triple avoids
  both problems.
- Letting `lo` or `hi` reuse index `i`: the three positions must differ.
