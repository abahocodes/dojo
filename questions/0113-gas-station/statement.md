Fuel stations `0, 1, ..., n - 1` sit on a circular road. Stopping at station
`i` lets you add `gas[i]` units of fuel, and driving from station `i` to the
next one (`i + 1`, or `0` after the last) burns `cost[i]` units.

Your tank holds unlimited fuel but starts **empty**. You pick a starting
station, fill up there, and drive clockwise, refuelling at every station you
reach. The tank may hit exactly `0` but may never go negative. Return the
index of a starting station from which you can drive all the way around and
back to it. If several stations work, return the **smallest** such index. If
none works, return `-1`.

## Example 1

```
gas    = [2, 5, 1, 5]
cost   = [3, 3, 4, 2]
output = 3
# start at 3: tank 5-2=3 -> +2-3=2 -> +5-3=4 -> +1-4=1, back at 3
```

## Example 2

```
gas    = [3, 1, 2]
cost   = [2, 3, 2]
output = -1       # total fuel 6 < total cost 7
```

## Constraints

- `1 <= n == len(gas) == len(cost) <= 10^5`
- `0 <= gas[i], cost[i] <= 10^4`
