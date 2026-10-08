# Approach: three pointers, filling from the back

In the classic in-place version, `nums1` has room for `m + n` values with
its last `n` slots empty. Filling from the front would overwrite values of
`nums1` that have not been placed yet; filling from the back never does,
because the write pointer `w = i + j + 1` is always at or ahead of `i`.

Keep `i` at the last unplaced element of `nums1`, `j` at the last of
`nums2`, and `w` at the next slot to write. At each step write the larger
tail element. Once `nums2` is used up, the remaining prefix of `nums1` is
already where it belongs.

```python
def merge_sorted(nums1, nums2):
    m, n = len(nums1), len(nums2)
    out = nums1 + [0] * n          # nums1 with n empty slots at the end
    i, j, w = m - 1, n - 1, m + n - 1
    while j >= 0:
        if i >= 0 and out[i] > nums2[j]:
            out[w] = out[i]
            i -= 1
        else:
            out[w] = nums2[j]
            j -= 1
        w -= 1
    return out
```

## Complexity

- Time: O(m + n), each element is written once.
- Space: O(1) beyond the output array.

## Pitfalls

- Merging from the front in place, which overwrites unread elements of
  `nums1`.
- Looping while `i >= 0 or j >= 0` and then reading `nums1[-1]` (or out of
  bounds) when `i` runs out.
- Forgetting the empty-array cases: either input may have length 0.
