# Approach: binary search on the answer

Let `total(d) = sum(ceil(x / d) for x in nums)`. Increasing `d` never increases
any term, so `total` is non-increasing. That makes the predicate
"`total(d) <= threshold`" monotone: false for small divisors, then true from
some point on. The answer is the first divisor where it becomes true.

The search range is `[1, max(nums)]`: with `d = max(nums)` every term is `1`,
so `total(d) = len(nums) <= threshold`, and larger divisors cannot do better.
Each step evaluates `total` in O(n) using the integer form of ceiling
division, `(x + d - 1) // d`.

```python
def smallest_divisor(nums, threshold):
    def total(d):
        return sum((x + d - 1) // d for x in nums)

    lo, hi = 1, max(nums)
    while lo < hi:
        mid = (lo + hi) // 2
        if total(mid) <= threshold:
            hi = mid          # mid works; the answer is mid or smaller
        else:
            lo = mid + 1      # mid is too small
    return lo
```

## Complexity

- Time: O(n log M), where `M = max(nums)` (about 20 iterations for `10^6`).
- Space: O(1).

## Pitfalls

- Using floor division instead of ceiling division.
- Overflow: with `d = 1` the sum is up to `5 * 10^4 * 10^6 = 5 * 10^10`, which
  does not fit in a 32-bit int. Accumulate in a 64-bit type.
- Starting the search at `0`, which divides by zero.
