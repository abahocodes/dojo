# Approach: look for a far-apart inversion

Every local inversion is a global inversion, so the two counts are equal if
and only if no global inversion is *non-local*, i.e. there is no pair
`i < j` with `j - i >= 2` and `nums[i] > nums[j]`.

For each `j`, the best candidate `i` is the one holding the largest value
among `nums[0..j-2]`. Scan left to right, maintaining that running maximum,
and fail as soon as it exceeds `nums[j]`. No inversion counting is needed.

```python
def is_ideal_permutation(nums):
    best = -1
    for j in range(2, len(nums)):
        best = max(best, nums[j - 2])
        if best > nums[j]:
            return False
    return True
```

Because `nums` is a permutation of `0..n-1`, an equivalent test is
`abs(nums[i] - i) <= 1` for every `i`: a value displaced by two or more
positions forces a non-local inversion.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Counting global inversions with a double loop: O(n^2) is far too slow for
  `n = 10^5`. (A merge-sort count works in O(n log n) but is unnecessary.)
- Including `nums[j - 1]` in the running maximum: an adjacent pair is a local
  inversion and must not fail the test.
- Forgetting that `n` can be 1 or 2; both always give `true`.
