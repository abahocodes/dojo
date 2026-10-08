# Approach: count prefix remainders

Let `P(i)` be the sum of the first `i` elements (`P(0) = 0`). The subarray
`nums[j..i-1]` sums to `P(i) - P(j)`, which is divisible by `k` exactly when
`P(i) ≡ P(j) (mod k)`. So the answer is the number of pairs `j < i` of prefix
positions whose remainders match.

Walk the array once, keeping `count[r]`, the number of prefixes seen so far
with remainder `r`. When the new prefix has remainder `r`, it forms a valid
pair with each of those `count[r]` earlier prefixes.

```python
def subarrays_div_by_k(nums, k):
    count = [0] * k
    count[0] = 1          # the empty prefix
    rem = 0
    result = 0
    for x in nums:
        rem = (rem + x) % k
        result += count[rem]
        count[rem] += 1
    return result
```

Python's `%` always returns a value in `[0, k)`. JavaScript, TypeScript, Java,
C++ and Go keep the sign of the left operand, so they compute
`((rem + x) % k + k) % k` instead.

## Complexity

- Time: O(n + k).
- Space: O(k) for the remainder counts.

## Pitfalls

- Negative remainders. Without normalising, `-1` and `3` (mod 4) land in
  different buckets even though they are the same class.
- Forgetting `count[0] = 1`: subarrays that start at index `0` are missed.
- Adding `count[rem]` **after** incrementing it, which counts each prefix
  paired with itself.
- The answer can be as large as `n(n+1)/2 ≈ 4.5 * 10^8`, which still fits a
  32-bit int.
