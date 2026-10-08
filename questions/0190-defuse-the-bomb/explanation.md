# Approach: a sliding window around the ring

Every position uses a window of `|k|` consecutive ring elements, and the window
for position `i + 1` is the window for `i` shifted by one. Pick the window's
bounds `[start, end]` for position `0`:

- `k > 0`: `start = 1`, `end = k`;
- `k < 0`: `start = n + k`, `end = n - 1`.

Compute that sum once. For each next position, add `code[(end + 1) % n]` and
subtract `code[start % n]`, then advance both bounds. `k == 0` is just zeros.

```python
def decrypt(code, k):
    n = len(code)
    result = [0] * n
    if k == 0:
        return result
    start, end = (1, k) if k > 0 else (n + k, n - 1)
    window = sum(code[j % n] for j in range(start, end + 1))
    for i in range(n):
        result[i] = window
        window -= code[start % n]
        start += 1
        end += 1
        window += code[end % n]
    return result
```

The output goes into a separate array, so every replacement reads the original
values: the change is simultaneous, as required.

## Complexity

- Time: O(n), after an O(|k|) first sum.
- Space: O(n) for the result.

## Pitfalls

- Overwriting `code` in place, so later positions read already-replaced values.
- Negative indices: in Java, C++, Go and JavaScript `-1 % n` is `-1`, not
  `n - 1`. Keep indices non-negative before taking the modulus.
- The window for `k < 0` excludes `code[i]` itself, just like the window for
  `k > 0`.
