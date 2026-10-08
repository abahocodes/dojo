# Approach: sliding window over the product

Because every element is at least 1, the product of a window only grows as the
window grows. For each right end there is therefore a smallest left end
`left` such that `nums[left..right]` has product below `k`, and every start
between `left` and `right` also works. That gives `right - left + 1` subarrays
ending at `right`. As `right` advances, `left` never moves back, so a running
product and two pointers find all of them.

```python
def num_subarray_product_less_than_k(nums, k):
    if k <= 1:
        return 0
    product = 1
    left = total = 0
    for right, x in enumerate(nums):
        product *= x
        while product >= k:
            product //= nums[left]
            left += 1
        total += right - left + 1
    return total
```

## Complexity

- Time: O(n). Each index enters and leaves the window once.
- Space: O(1).

## Pitfalls

- `k <= 1`: no product can be below it. Without the early return, the
  shrinking loop runs `left` past `right` and divides by elements that are no
  longer in the window.
- Overflow: the product stays below `k * 1000 <= 10^9` before shrinking, but
  use a 64-bit type in Java and C++ to be safe.
- The answer can reach `n * (n + 1) / 2`, about `4.5 * 10^8`, which still fits
  in a 32-bit integer.
