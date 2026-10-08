You must take `n` trains one after another to get to work, and you have
`hour` hours in total. Train `i` covers `dist[i]` kilometres. Every train
travels at the same speed `s` (km per hour), which you choose; it must be a
positive integer.

Riding train `i` takes `dist[i] / s` hours (a fraction is fine). However,
every train after the first **departs only on a whole hour**: when a ride ends
at a fractional time you wait until the next integer hour before boarding the
next train. No waiting is needed after the last train, and no waiting is
needed if a ride ends exactly on an integer hour.

Return the **minimum** speed `s` that gets you to work in at most `hour` hours,
or `-1` if no speed can. Whenever some speed works, a speed of at most `10^7`
works.

## Example 1

```
dist   = [2, 5, 3]
hour   = 4.5
output = 3     # 2/3 -> wait to 1, 5/3 -> wait to 3, then 3/3 = 1 more: arrive at 4
               # speed 2 arrives at 1 + 3 + 1.5 = 5.5
```

## Example 2

```
dist   = [1, 1, 1]
hour   = 1.9
output = -1    # the first two trains use at least two whole hours
```

## Constraints

- `1 <= n <= 10^5`
- `1 <= dist[i] <= 10^5`
- `0 < hour <= 10^9`, and `hour` has at most two digits after the decimal
  point
