# Approach: sentinels on both sides

Missing integers come in blocks that sit between two consecutive present
values. The blocks at the very start and very end of the range are the awkward
ones, so remove the special cases with two sentinels: treat `lower - 1` as the
"previous" present value before the scan starts, and `upper + 1` as one more
present value after `nums` ends.

Then, for every consecutive pair `(prev, x)` of the extended sequence, the
integers strictly between them are missing. There are some exactly when
`x - prev >= 2`, and the block is `[prev + 1, x - 1]`. Because the scan moves
left to right, the blocks come out already sorted.

```python
def find_missing_ranges(nums, lower, upper):
    ranges = []
    prev = lower - 1
    for x in nums + [upper + 1]:
        if x - prev >= 2:
            ranges.append([prev + 1, x - 1])
        prev = x
    return ranges
```

## Complexity

- Time: O(n), one pass over `nums`.
- Space: O(1) besides the output.

## Pitfalls

- Forgetting the gap before `nums[0]` or after `nums[-1]`, or the whole range
  when `nums` is empty.
- Reporting `[x, x]` blocks as anything else: a single missing integer is still
  a pair with both ends equal.
- Overflow in the sentinels. With these bounds `lower - 1` and `upper + 1` fit
  in 32 bits, but they would not for inputs near `±2^31`; the Java and C++
  solutions hold them in 64-bit variables so the code stays safe.
