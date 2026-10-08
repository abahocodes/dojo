# Approach: binary search on the product value

Let `count(x)` be the number of pairs with `nums1[i] * nums2[j] <= x`. It is
non-decreasing in `x`, so the answer is the smallest `x` with
`count(x) >= k`. Every product lies between the smallest and largest of the
four corner products, which bounds the search.

To compute `count(x)`, handle each `a` in `nums1` on its own:

- `a > 0`: `a * b <= x` exactly when `b <= floor(x / a)`; count the prefix of
  `nums2` with a binary search.
- `a < 0`: dividing by a negative flips the inequality: `b >= ceil(x / a)`;
  count the suffix.
- `a == 0`: every product is 0, so all of `nums2` counts if `x >= 0`.

```python
from bisect import bisect_left, bisect_right

def kth_smallest_product(nums1, nums2, k):
    n2 = len(nums2)

    def count_at_most(x):
        total = 0
        for a in nums1:
            if a > 0:
                total += bisect_right(nums2, x // a)
            elif a < 0:
                total += n2 - bisect_left(nums2, -(-x // a))
            elif x >= 0:
                total += n2
        return total

    corners = [nums1[0] * nums2[0], nums1[0] * nums2[-1],
               nums1[-1] * nums2[0], nums1[-1] * nums2[-1]]
    lo, hi = min(corners), max(corners)
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

In languages whose integer division truncates toward zero, it is simpler to
binary search on the comparison `a * nums2[j] <= x` directly, as the other
reference solutions do.

## Complexity

- Time: O(n1 * log n2 * log R), where R (about 2 * 10^10) is the range of
  products.
- Space: O(1).

## Pitfalls

- Floor and ceiling with negative numbers: in C++, Java, Go and JavaScript,
  `/` truncates toward zero, so `x / a` is not `floor(x / a)` when signs differ.
- Products and `k` overflow 32-bit integers.
- Midpoints of a range with negative ends: `(lo + hi) / 2` truncates toward
  zero in many languages; use `lo + (hi - lo) / 2` or floor division so the
  search still converges.
- A heap that merges rows works only for all-positive inputs and costs
  O(k log n), with `k` up to 2.5 * 10^9.
