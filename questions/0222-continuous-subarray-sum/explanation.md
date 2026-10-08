# Approach: equal prefix remainders

The sum of `nums[j+1..i]` is `P(i) - P(j)`, where `P(i)` is the sum of
`nums[0..i]` and `P(-1) = 0`. That difference is a multiple of `k` exactly when
`P(i)` and `P(j)` leave the same remainder modulo `k`. The subarray has
`i - j` elements, so we need two prefix positions `j < i` with the same
remainder and `i - j >= 2`.

Record the first position at which each remainder appears. The earliest
position gives the longest candidate, so if it is not far enough back, no
other position with that remainder is either.

```python
def check_subarray_sum(nums, k):
    first = {0: -1}
    rem = 0
    for i, x in enumerate(nums):
        rem = (rem + x) % k
        if rem in first:
            if i - first[rem] >= 2:
                return True
        else:
            first[rem] = i
    return False
```

The running value is reduced modulo `k` on every step, so it stays below `k`.
`rem + x` can still reach about `2^31 + 10^9`, which overflows a 32-bit int: the
Java and C++ solutions keep it in a 64-bit variable.

## Complexity

- Time: O(n).
- Space: O(min(n, k)) for the remainder table.

## Pitfalls

- Overwriting the stored position when a remainder repeats. With
  `nums = [5, 0, 0]`, `k = 9`, remainder `5` appears at positions 0, 1 and 2;
  keeping only the latest one makes every gap 1, so the subarray `[0, 0]` is
  missed. Keep the first.
- Forgetting the length rule: a single element that is a multiple of `k` (or a
  single `0`) does not count.
- Forgetting the starting entry `0 -> -1`, which handles subarrays that begin
  at index `0`.
- Summing without reducing: the full sum can reach `10^14`, past 32 bits.
