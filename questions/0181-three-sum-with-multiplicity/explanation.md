# Approach: count values, then combine

A triple of positions is determined, as far as the sum is concerned, only by
its three values. Count how often each value `0..100` occurs, then walk over
sorted value triples `x <= y <= z` with `z = target - x - y`. Each one
contributes a number of position triples that depends on which values are
equal:

| shape        | position triples            |
|--------------|-----------------------------|
| `x < y < z`  | `c[x] * c[y] * c[z]`        |
| `x == y < z` | `C(c[x], 2) * c[z]`         |
| `x < y == z` | `c[x] * C(c[y], 2)`         |
| `x == y == z`| `C(c[x], 3)`                |

Requiring `x <= y <= z` makes every multiset of values appear once, so no
triple is counted twice.

```python
MOD = 10**9 + 7

def three_sum_multi(arr, target):
    c = [0] * 101
    for v in arr:
        c[v] += 1
    total = 0
    for x in range(101):
        for y in range(x, 101):
            z = target - x - y
            if z < y or z > 100:
                continue
            if x == y == z:
                total += c[x] * (c[x] - 1) * (c[x] - 2) // 6
            elif x == y:
                total += c[x] * (c[x] - 1) // 2 * c[z]
            elif y == z:
                total += c[x] * c[y] * (c[y] - 1) // 2
            else:
                total += c[x] * c[y] * c[z]
    return total % MOD
```

A sort-plus-two-pointer solution in the style of 3Sum also works in
O(n^2), but it needs careful handling of runs of equal values; counting
avoids that entirely.

## Complexity

- Time: O(n + V^2) with `V = 101` distinct values.
- Space: O(V) for the counts.

## Pitfalls

- Overflow: `c[x] * c[y] * c[z]` can reach 2.7 * 10^10 and `C(3000, 3)` is
  about 4.5 * 10^9. Use 64-bit integers and reduce modulo `10^9 + 7`.
- Counting each value triple once per ordering. Enforce `x <= y <= z`.
- `z` can be negative or above 100; skip those instead of indexing out of
  range.
- Using `c * c * c` for all-equal triples instead of `C(c, 3)`: the three
  positions must be distinct.
