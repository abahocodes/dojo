# Approach: binary search on the running time

For a target of `T` minutes, a battery can contribute at most
`min(b, T)` minutes, since it can power only one computer per minute. We need
`n * T` battery-minutes in total, so `sum(min(b, T)) >= n * T` is necessary.

It is also sufficient. Think of `n` columns of height `T` and pour the capped
batteries in one after another, wrapping from the top of one column to the
bottom of the next. A battery of length at most `T` never overlaps itself in
time, so it never powers two computers at once.

If `T` is feasible, any smaller `T` is too, so binary search for the largest
feasible value. `sum(batteries) // n` is an upper bound.

```python
def max_run_time(n, batteries):
    def can_run(minutes):
        return sum(min(b, minutes) for b in batteries) >= n * minutes

    lo, hi = 0, sum(batteries) // n
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if can_run(mid):
            lo = mid
        else:
            hi = mid - 1
    return lo
```

An alternative with the same idea: sort descending, and while the largest
battery exceeds `total // n`, dedicate it to one computer and drop both
(`total -= b`, `n -= 1`). The answer is then `total // n`.

## Complexity

- Time: O(m log(S / n)), where m is `len(batteries)` and S their sum.
- Space: O(1).

## Pitfalls

- Answers and sums reach about `10^14`: use 64-bit integers for `total`,
  `n * T` and the running sum.
- When searching for the largest feasible value, round the midpoint up
  (`(lo + hi + 1) // 2`) or the loop can stall.
- Assigning each battery to one computer forever (no swapping) gives a smaller
  answer; Example 1 needs the shared third battery to reach 6.
