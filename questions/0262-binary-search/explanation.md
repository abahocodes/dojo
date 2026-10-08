# Approach: halve the search window

Because `nums` is sorted, one comparison with the middle element tells you
which half cannot contain `target`. Keep the inclusive window `[lo, hi]` of
candidate indices and halve it each step.

```python
def search(nums, target):
    lo, hi = 0, len(nums) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if nums[mid] == target:
            return mid
        if nums[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1
```

Invariant: if `target` is in the array, its index lies in `[lo, hi]`. Each
step either finds it or removes `mid` plus one half, so the loop ends after
about `log2(n)` iterations.

## Complexity

- Time: `O(log n)`.
- Space: `O(1)`.

## Pitfalls

- Writing `lo = mid` or `hi = mid` with an inclusive window: when
  `lo == hi - 1` the window may stop shrinking and the loop never ends.
- Using `while lo < hi` with an inclusive window and forgetting to check the
  last remaining element.
- `(lo + hi) / 2` can overflow 32-bit integers for huge arrays in Java/C++;
  `lo + (hi - lo) / 2` is the safe habit (it does not matter at `10^5`).
- A linear scan is correct but `O(n)`, which the problem rules out.
