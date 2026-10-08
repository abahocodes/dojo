# Approach: sort, then two pointers with powers of two

The condition depends only on the smallest and largest chosen values, and the
number of subsequences with a given multiset of values is the same no matter
how the array is ordered. So sort the array.

In the sorted array `a`, count the subsequences by their first (smallest)
position `lo`. If `a[lo] + a[hi] <= target` for the largest such `hi`, then
any subset of positions `lo+1..hi` added to `lo` is valid: `2^(hi - lo)`
subsequences. Moving `lo` right only makes `a[lo]` larger, so the matching
`hi` never moves right again. That gives a two-pointer sweep:

- if `a[lo] + a[hi] <= target`: add `2^(hi - lo)` and advance `lo`;
- otherwise `a[hi]` is too big for every remaining `lo`; move `hi` left.

```python
MOD = 10**9 + 7

def num_subseq(nums, target):
    a = sorted(nums)
    n = len(a)
    pow2 = [1] * n
    for i in range(1, n):
        pow2[i] = pow2[i - 1] * 2 % MOD
    total = 0
    lo, hi = 0, n - 1
    while lo <= hi:
        if a[lo] + a[hi] <= target:
            total = (total + pow2[hi - lo]) % MOD
            lo += 1
        else:
            hi -= 1
    return total
```

## Complexity

- Time: O(n log n) for the sort; the sweep is O(n).
- Space: O(n) for the sorted copy and the table of powers of two.

## Pitfalls

- Forgetting single-element subsequences: `[x]` counts when `2 * x <= target`.
  The loop handles it with `lo == hi` and `2^0 = 1`, so keep `<=` in the loop
  condition.
- Computing `2^(hi - lo)` with floating point or plain shifts: exponents reach
  10^5. Precompute powers modulo `10^9 + 7`.
- Overflow in Java and C++: `total + pow2[k]` can exceed 2^31 before the
  modulus; keep sums in 64-bit integers or reduce each time.
- Counting by distinct values instead of positions: `[1, 1, 1]` with target 2
  has 7 subsequences, not 1.
