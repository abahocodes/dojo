# Approach: mark presence with signs, in place

Every value `v` is in `[1, n]`, so `v - 1` is a valid index. Use the sign of
`nums[v - 1]` as a "seen `v`" flag: for each element, read its absolute value
(it may already have been negated) and make the slot it points to negative.
Afterwards, a slot `i` that is still positive was never pointed to, so `i + 1`
is missing. Reading the slots in index order gives the answer already sorted.

```python
def find_disappeared_numbers(nums):
    for x in nums:
        i = abs(x) - 1
        if nums[i] > 0:
            nums[i] = -nums[i]
    return [i + 1 for i, x in enumerate(nums) if x > 0]
```

## Complexity

- Time: O(n), two passes.
- Space: O(1) besides the output (the input is modified).

## Pitfalls

- Using `x - 1` instead of `abs(x) - 1`: an element may already have been
  negated by an earlier step.
- Negating unconditionally, which flips a slot back to positive when its value
  is seen twice.
- Returning the missing values unsorted, for example from a set difference.
