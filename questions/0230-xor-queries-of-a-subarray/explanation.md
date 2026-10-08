# Approach: prefix XOR

Let `prefix[i]` be the XOR of the first `i` elements (`prefix[0] = 0`). For a
query `[l, r]`, `prefix[r + 1]` is the XOR of `arr[0..r]` and `prefix[l]` is
the XOR of `arr[0..l-1]`. XOR-ing them cancels the shared part, because
`x ^ x == 0` and XOR is associative and commutative, leaving exactly
`arr[l] ^ ... ^ arr[r]`.

```python
def xor_queries(arr, queries):
    prefix = [0] * (len(arr) + 1)
    for i, x in enumerate(arr):
        prefix[i + 1] = prefix[i] ^ x
    return [prefix[r + 1] ^ prefix[l] for l, r in queries]
```

## Complexity

- Time: O(n + q) for `n` elements and `q` queries.
- Space: O(n) for the prefix array (plus the output).

## Pitfalls

- Using `prefix[r] ^ prefix[l - 1]` with a prefix array that has no leading
  zero, which breaks when `l == 0`.
- Subtracting instead of XOR-ing: the inverse of XOR is XOR.
- Overflow is impossible: every value is below 2^30, so every XOR is too.
