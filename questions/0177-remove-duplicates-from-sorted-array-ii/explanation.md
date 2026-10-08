# Approach: write pointer comparing two slots back

Scan `nums` with a read pointer and keep a write pointer `k`. The output so
far is `nums[:k]` and it is sorted. A new value `x` would be a third copy
exactly when the output already ends with `x, x`, i.e. when
`nums[k - 2] == x` (sortedness guarantees `nums[k - 1]` is then also `x`).
So write `x` at position `k` whenever `k < 2` or `nums[k - 2] != x`.

The write pointer never overtakes the read pointer, so overwriting in place is
safe. The solutions below work on a copy so the caller's array is untouched.

```python
def remove_duplicates_keep_two(nums):
    a = list(nums)
    k = 0
    for x in a:
        if k < 2 or a[k - 2] != x:
            a[k] = x
            k += 1
    return a[:k]
```

The same idea generalises to "at most `m` copies": compare with `a[k - m]`.

## Complexity

- Time: O(n), one pass.
- Space: O(1) extra beyond the copy that is returned.

## Pitfalls

- Comparing with `a[k - 1]` instead of `a[k - 2]`: that keeps one copy, not
  two.
- Comparing with the *input* element two places back (`nums[i - 2]`) after
  overwriting in place: once elements have shifted, the read side no longer
  reflects what was kept.
- Forgetting that the first two elements are always kept.
