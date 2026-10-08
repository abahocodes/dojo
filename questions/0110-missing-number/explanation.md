# Approach: XOR the indices against the values

If we XOR together all numbers `0..n` and all values in `nums`, every number
that is present appears twice and cancels out. Only the removed number appears
once, so it is what remains.

```python
def missing_number(nums):
    result = len(nums)
    for i, x in enumerate(nums):
        result ^= i ^ x
    return result
```

Starting `result` at `n` covers the one index (`n`) that `enumerate` never
produces.

**Alternative (sum):** `n * (n + 1) // 2 - sum(nums)`. In languages with fixed
size integers it can overflow for large `n`; XOR never does.

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Forgetting that the missing number may be `n` (then every index `0..n-1` is
  present) or `0`.
- A set of the values works but uses O(n) memory.
- Sorting changes the input and costs O(n log n).
