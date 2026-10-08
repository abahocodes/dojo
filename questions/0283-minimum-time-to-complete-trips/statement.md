A fleet of buses runs on a shuttle route. Bus `i` needs `time[i]` units of
time to complete one trip, and as soon as it finishes a trip it starts the
next one. All buses start at time `0` and operate independently of each
other, so by time `t` bus `i` has completed `floor(t / time[i])` trips.

Return the minimum time `t` at which the buses, counted together, have
completed at least `total_trips` trips.

## Example 1

```
time        = [1, 2, 3]
total_trips = 5
output      = 3   # by t = 3: 3 + 1 + 1 = 5 trips; by t = 2 only 2 + 1 + 0 = 3
```

## Example 2

```
time        = [4]
total_trips = 3
output      = 12
```

## Constraints

- `1 <= len(time) <= 10^5`
- `1 <= time[i] <= 10^7`
- `1 <= total_trips <= 10^7`
- The answer can exceed `2^31 - 1`; return it as a 64-bit integer.
