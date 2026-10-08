# Approach: smallest prefix sum

After the first `i + 1` additions the total is `start + prefix_i`, where
`prefix_i = nums[0] + ... + nums[i]`. All of these must be `>= 1`, which is
the same as `start >= 1 - min(prefix_i)`. Combined with `start >= 1`:

    answer = max(1, 1 - min prefix sum)

```python
def min_start_value(nums):
    total = 0
    low = 0
    for x in nums:
        total += x
        low = min(low, total)
    return 1 - low
```

Starting `low` at `0` handles the "at least 1" rule: if every prefix sum is
positive, `low` stays `0` and the answer is `1`.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Returning `-min prefix` instead of `1 - min prefix`. The total must stay at
  least `1`, not at least `0`.
- Returning `0` or a negative value when all prefix sums are positive. The
  start value must be positive.
- Trying start values one by one works for these bounds but is needless.
