# Approach: binary search on the value

Let `count(x)` be the number of cells whose value is at most `x`. Row `i`
contains the multiples `i, 2i, ..., n*i`, so exactly `min(x // i, n)` of them are
at most `x`. Summing over the rows gives `count(x)` in O(m) time (iterate over
the shorter side).

`count` is non-decreasing, and the answer is the smallest `x` with
`count(x) >= k`. That smallest `x` is guaranteed to appear in the table:
`count(x) > count(x - 1)` means some cell equals `x`.

```python
def find_kth_number(m, n, k):
    if m > n:
        m, n = n, m

    def count_at_most(x):
        full_rows = min(m, x // n)      # rows whose every cell is <= x
        total = full_rows * n
        for i in range(full_rows + 1, min(m, x) + 1):
            total += x // i
        return total

    lo, hi = 1, m * n
    while lo < hi:
        mid = (lo + hi) // 2
        if count_at_most(mid) >= k:
            hi = mid
        else:
            lo = mid + 1
    return lo
```

## Complexity

- Time: O(min(m, n) * log(m * n)).
- Space: O(1).

## Pitfalls

- Searching for the first `x` with `count(x) == k` fails when the answer is
  duplicated: the count can jump past `k`. Use `>= k`.
- Forgetting the `min(..., n)` cap counts cells that don't exist.
- Sorting or heap-merging the table costs O(mn) or O(k log m), hopeless at
  `9 * 10^8` cells.
- `m * n` reaches `9 * 10^8`, which fits a 32-bit int, but compute midpoints as
  `lo + (hi - lo) / 2` to stay safe.
