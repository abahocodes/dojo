A staircase has `n` steps, numbered `0` to `n - 1`, and stepping off step
`i` costs `cost[i]`. When you pay for a step you may then climb either one
or two steps up. You may begin on step `0` or on step `1` for free, and you
are done when you get past the last step, to the landing just above step
`n - 1`.

Return the smallest total cost of getting to the landing.

## Example 1

```
cost   = [10, 15, 20]
output = 15
```

Start on step `1`, pay `15` and climb two steps, which reaches the landing.

## Example 2

```
cost   = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
output = 6
```

Start on step `0` and pay `1` at steps `0, 2, 4, 6, 7, 9`, always skipping
the steps that cost `100`.

## Constraints

- `2 <= len(cost) <= 1000`
- `0 <= cost[i] <= 999`
