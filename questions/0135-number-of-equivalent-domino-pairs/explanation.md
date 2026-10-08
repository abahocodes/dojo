# Approach: canonical key and running counts

Two dominoes are equivalent exactly when their sorted pairs are equal, so map
`[a, b]` to the key `10 * min(a, b) + max(a, b)` (values are 1..9, so the key
is below 100 and unique per sorted pair). Scan the dominoes, and for each one
add the number of earlier dominoes with the same key, then increment that
count. This counts every pair `i < j` exactly once, at `j`.

```python
def num_equiv_domino_pairs(dominoes):
    seen = [0] * 100
    pairs = 0
    for a, b in dominoes:
        key = 10 * min(a, b) + max(a, b)
        pairs += seen[key]
        seen[key] += 1
    return pairs
```

Equivalently, a group of `k` equal keys contributes `k * (k - 1) / 2` pairs.

## Complexity

- Time: O(n).
- Space: O(1): 100 counters.

## Pitfalls

- Using `10 * a + b` without sorting, which separates `[3, 5]` from `[5, 3]`.
- The O(n^2) pair check times out at the upper bound.
- The answer can reach about 8 * 10^8 (all 40,000 dominoes equal), which
  still fits in a 32-bit int.
