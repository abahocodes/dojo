# Approach: contribution counting with monotonic stacks

`sum(max - min) = sum(max) - sum(min)` over all subarrays, so it suffices to
compute the sum of subarray maxima and the sum of subarray minima.

For the maxima, credit each subarray to its **rightmost** maximum. Index `j` is
that rightmost maximum for every subarray whose left end lies in
`(L, j]` and whose right end lies in `[j, R)`, where `L` is the nearest index to
the left with a value **strictly greater** than `nums[j]` and `R` the nearest
index to the right with a value **greater than or equal to** it. That is
`(j - L) * (R - j)` subarrays.

A stack of indices with strictly decreasing values discovers these bounds: when
index `i` arrives (or the virtual end `i = n`), every index `j` on top with
`nums[j] <= nums[i]` is popped; its right bound is `i` and its left bound is
the index left below it on the stack (or `-1`). Minima are symmetric with the
comparison flipped.

```python
def sub_array_ranges(nums):
    n = len(nums)

    def total(better):
        # Sum over subarrays of the extreme value, where better(a, b) is True
        # when a should knock b off the stack.
        result = 0
        stack = []
        for i in range(n + 1):
            while stack and (i == n or not better(nums[stack[-1]], nums[i])):
                j = stack.pop()
                left = stack[-1] if stack else -1
                result += nums[j] * (j - left) * (i - j)
            stack.append(i)
        return result

    return total(lambda a, b: a > b) - total(lambda a, b: a < b)
```

## Complexity

- Time: O(n) — each index is pushed and popped once per pass.
- Space: O(n) for the stack.

## Pitfalls

- Tie handling: using "strictly greater" on both sides undercounts subarrays
  with repeated maxima, and "greater or equal" on both sides double counts
  them. One side must be strict, the other not.
- Overflow: `(j - left) * (i - j)` can reach `2.5 * 10^9`, already past 32 bits,
  and multiplying by the value goes further. Use 64-bit arithmetic for the
  product (cast *before* multiplying in Java/C++).
- The O(n^2) "extend each subarray while tracking min and max" approach is
  correct but too slow for `n = 10^5`.
