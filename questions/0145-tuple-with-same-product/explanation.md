# Approach: count pair products

Every valid tuple consists of two unordered pairs `{a, b}` and `{c, d}` with
equal products. Since the values are distinct, two different pairs with the
same product must be disjoint: if they shared `a`, then `a * b == a * d`
would force `b == d`. So any two different pairs with equal products give
valid tuples, in `8` orders (swap inside the first pair, swap inside the
second, swap the pairs).

Enumerate all `n * (n - 1) / 2` pairs and count their products. When a pair's
product has already been seen `m` times, it forms `m` new pairs of pairs,
each worth 8 tuples.

```python
def tuple_same_product(nums):
    seen = {}
    total = 0
    for i in range(len(nums)):
        for j in range(i + 1, len(nums)):
            p = nums[i] * nums[j]
            total += 8 * seen.get(p, 0)
            seen[p] = seen.get(p, 0) + 1
    return total
```

## Complexity

- Time: O(n^2) expected: one hash-map update per pair.
- Space: O(n^2) in the worst case for the map of products.

## Pitfalls

- Forgetting the factor 8 (or using 4, which only counts one order of the
  two pairs).
- Iterating `j` from `0` instead of `i + 1`: that counts `(a, a)` pairs and
  each unordered pair twice.
- Products reach `10^8`, which fits in 32 bits, so no wider type is needed for
  the key itself.
