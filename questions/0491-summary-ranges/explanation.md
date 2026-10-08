# Approach: one pass over maximal runs

Since `nums` is strictly increasing, the numbers that can share a range are
exactly the maximal runs of consecutive integers. A run ends at index `j`
whenever `nums[j + 1] != nums[j] + 1`, because the gap between them is
missing from `nums` and no range may cover it. Splitting a run into more
pieces would only add ranges, so taking each maximal run as one range gives
the smallest list.

Walk with two indices: `i` marks the start of the current run and `j`
advances while the next number continues it. Emit `"nums[i]"` or
`"nums[i]->nums[j]"`, then start the next run at `j + 1`.

```python
def summary_ranges(nums):
    result = []
    i = 0
    while i < len(nums):
        j = i
        while j + 1 < len(nums) and nums[j + 1] == nums[j] + 1:
            j += 1
        result.append(str(nums[i]) if i == j else f"{nums[i]}->{nums[j]}")
        i = j + 1
    return result
```

## Complexity

- Time: O(n): each index is visited once by `j`.
- Space: O(1) besides the output.

## Pitfalls

- Integer overflow in 32-bit languages: `nums[j + 1] - nums[j]` overflows
  for `-2^31` followed by `2^31 - 1`. Checking `nums[j + 1] == nums[j] + 1`
  is safe because `nums[j] < nums[j + 1] <= 2^31 - 1`.
- Writing a single number as `"a->a"`.
- Forgetting to flush the last run after the loop when using a
  "close on gap" style loop.
- Negative numbers: `"-3->-1"` is the correct format; the minus sign is part
  of the number.
