# Approach: prefix and suffix products

The product of all other elements splits into the product of what lies to the
left times the product of what lies to the right. Fill `out` with prefix
products in one pass, then sweep backwards multiplying in suffix products held
in a single variable.

```python
def product_except_self(nums):
    n = len(nums)
    out = [1] * n
    prefix = 1
    for i in range(n):
        out[i] = prefix
        prefix *= nums[i]
    suffix = 1
    for i in range(n - 1, -1, -1):
        out[i] *= suffix
        suffix *= nums[i]
    return out
```

## Complexity

- Time: O(n) — two linear passes.
- Space: O(1) extra beyond the output list.

## Pitfalls

- The division trick (`total // nums[i]`) is disallowed and breaks on zeros anyway.
- Including `nums[i]` in its own prefix/suffix: assign before multiplying.
- Zeros: with one zero, only that position is non-zero; with two or more,
  everything is 0. The prefix/suffix method handles both without special cases.
- Alternative: separate `left` and `right` arrays — clearer, but O(n) extra space.
