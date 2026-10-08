# Approach: build on smaller answers

`i >> 1` is `i` with its last binary digit removed. That removes one `1` bit
exactly when `i` is odd. So the count for `i` is the count for `i >> 1`, plus
`1` if `i` is odd. Since `i >> 1 < i` for every `i >= 1`, that value is already
in the list when we need it.

```python
def count_bits(n):
    bits = [0] * (n + 1)
    for i in range(1, n + 1):
        bits[i] = bits[i >> 1] + (i & 1)
    return bits
```

**Alternative:** `i & (i - 1)` clears the lowest set bit of `i`, so
`bits[i] = bits[i & (i - 1)] + 1` also works.

## Complexity

- Time: O(n): constant work per entry.
- Space: O(n) for the output and nothing else.

## Pitfalls

- The list has `n + 1` entries; `n = 0` must return `[0]`.
- Using `bin(i).count("1")` per number is correct but misses the O(n) idea the
  question asks for.
- In JavaScript, `i >> 1` and `i & 1` are fine here because `n` stays far below
  `2^31`.
