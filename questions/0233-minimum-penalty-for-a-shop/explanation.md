# Approach: running penalty difference

The penalty at `j = 0` is the number of `'Y'` hours. Each step from `j` to
`j + 1` opens hour `j`: a `'Y'` there stops being a missed customer
(penalty `-1`), and an `'N'` becomes an idle open hour (penalty `+1`).

Since every candidate shares the same starting penalty, it is enough to track
the running change relative to `j = 0` and remember where it is smallest.
Updating only on a strictly smaller value keeps the earliest hour on ties.

```python
def best_closing_time(customers):
    delta = 0
    best_delta = 0
    best_hour = 0
    for i, c in enumerate(customers):
        delta += -1 if c == "Y" else 1
        if delta < best_delta:
            best_delta = delta
            best_hour = i + 1
    return best_hour
```

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Using `<=` when updating the best hour, which returns the *latest* hour
  among ties.
- Forgetting the candidates `j = 0` (never open) and `j = n` (open all day).
- Off-by-one: after reading hour `i`, the closing hour being scored is
  `i + 1`.
