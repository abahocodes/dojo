# Approach: prefix sums

Let `prefix[i]` be the sum of `nums[0..i-1]`, with `prefix[0] = 0`. Then
`nums[l] + ... + nums[r]` is everything up to `r` minus everything before `l`:

    prefix[r + 1] - prefix[l]

Building `prefix` takes one pass; every query is then a single subtraction.

```python
def range_sums(nums, queries):
    prefix = [0]
    for x in nums:
        prefix.append(prefix[-1] + x)
    return [prefix[r + 1] - prefix[l] for l, r in queries]
```

## Complexity

- Time: O(n + q).
- Space: O(n) for the prefix array (plus the O(q) output).

## Pitfalls

- Off-by-one: with the extra leading `0`, the range `[l, r]` is
  `prefix[r + 1] - prefix[l]`, not `prefix[r] - prefix[l]`.
- Overflow: `10^5` values of size `10^5` sum to `10^10`, beyond 32 bits. Use
  `long` / `long long` / Go `int` (64-bit) for the prefix sums.
- Summing each range in a loop is correct but O(n) per query.
