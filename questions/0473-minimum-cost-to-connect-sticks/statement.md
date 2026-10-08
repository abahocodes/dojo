You have a pile of sticks with positive integer lengths `sticks[i]`. In one
step you may pick any two sticks, of lengths `x` and `y`, and fuse them into a
single stick of length `x + y`. That step costs `x + y`.

Keep fusing until only one stick remains. Return the smallest possible total
cost. If you start with a single stick, nothing needs to be done and the cost
is `0`.

## Example 1

```
sticks = [4, 1, 6, 2]
output = 23
```

Fuse 1 and 2 (cost 3), then 3 and 4 (cost 7), then 7 and 6 (cost 13). The
total is 3 + 7 + 13 = 23, and no order does better.

## Example 2

```
sticks = [9]
output = 0
```

## Constraints

- `1 <= len(sticks) <= 10^4`
- `1 <= sticks[i] <= 10^4`
