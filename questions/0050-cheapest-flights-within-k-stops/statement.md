An airline serves `n` cities, numbered `0` to `n - 1`. Each entry
`[from, to, price]` in `flights` is a one-way flight from city `from` to city
`to` costing `price`.

You want to travel from city `src` to city `dst`, landing in **at most `k`
intermediate cities** along the way (so you take at most `k + 1` flights).
Return the lowest total price of such a trip, or `-1` if no trip respects the
limit.

## Example 1

```
n       = 4
flights = [[0, 1, 100], [1, 2, 100], [2, 3, 100], [0, 2, 500], [0, 3, 1000], [1, 3, 950]]
src = 0, dst = 3, k = 1
output  = 600   # 0 -> 2 -> 3; the 300 route 0 -> 1 -> 2 -> 3 needs 2 stops
```

## Example 2

```
n       = 4
flights = [[0, 1, 100], [1, 2, 100], [2, 3, 100], [0, 2, 500], [0, 3, 1000], [1, 3, 950]]
src = 0, dst = 3, k = 0
output  = 1000  # with no stops allowed, only the direct flight counts
```

## Constraints

- `2 <= n <= 100`
- `0 <= len(flights) <= n * (n - 1) / 2`
- `0 <= from, to < n` and `from != to`
- `1 <= price <= 10000`
- No pair `(from, to)` appears twice.
- `0 <= src, dst, k < n` and `src != dst`
