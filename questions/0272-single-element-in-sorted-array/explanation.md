# Approach: binary search on pair alignment

Before the single element, each pair starts at an even index:
`nums[0] == nums[1]`, `nums[2] == nums[3]`, and so on. The single element sits
at an even index too (an even number of elements precede it), and after it
every pair starts at an odd index. So for **even** `i`, the test
`nums[i] == nums[i + 1]` is true for all even indices before the single
element and false from the single element onward. The answer is the first even
index where it is false.

Binary search over that predicate. Keep the answer in `[lo, hi]` with both
ends even. Round `mid` down to an even index; if its pair is intact, the
answer is at `mid + 2` or later, otherwise at `mid` or earlier. `mid + 1` is
always in range because `mid < hi` and `hi` is the last index at most.

```python
def single_non_duplicate(nums):
    lo, hi = 0, len(nums) - 1
    while lo < hi:
        mid = (lo + hi) // 2
        if mid % 2 == 1:
            mid -= 1
        if nums[mid] == nums[mid + 1]:
            lo = mid + 2
        else:
            hi = mid
    return nums[lo]
```

## Complexity

- Time: O(log n).
- Space: O(1).

## Pitfalls

- Searching on values instead of indices. The values carry no information
  about where the single element is; the parity of positions does.
- Forgetting to align `mid` to an even index, which makes the comparison
  with `nums[mid + 1]` mean different things on different iterations.
- `lo = mid + 1` instead of `mid + 2`: that lands on an odd index and breaks
  the even-alignment invariant.
- The single element may be first, last, or the whole array (`n = 1`).
