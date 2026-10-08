# Approach: sign marking

Each value `v` corresponds to the index `v - 1`. We use the sign of
`nums[v - 1]` as a "seen" flag. The magnitude is untouched, so the original
value in that slot can still be read with `abs()`.

Scan the array. For each element take `v = abs(nums[i])`:

- if `nums[v - 1]` is negative, `v` was seen before, so it is a duplicate;
- otherwise flip `nums[v - 1]` to negative.

Since every value occurs at most twice, each duplicate is reported exactly
once.

```python
def find_duplicates(nums):
    result = []
    for x in nums:
        v = abs(x)
        if nums[v - 1] < 0:
            result.append(v)
        else:
            nums[v - 1] = -nums[v - 1]
    return result
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) beyond the output (the input is reused as the flag array).

## Pitfalls

- Reading `nums[i]` without `abs()`: the slot may already have been flipped
  by an earlier value, giving a negative index.
- Off-by-one: the value `v` maps to index `v - 1`, not `v`.
- Sorting first works but costs O(n log n).
