A creature is hit by a sequence of poison attacks. An attack at second `t`
poisons it for the `duration` seconds `t, t + 1, ..., t + duration - 1`. If a
new attack lands while the creature is still poisoned, the timer **restarts**:
the poison now lasts until `t' + duration - 1`, where `t'` is the time of the
new attack. Poison effects never stack.

Given the attack times `time_series` in non-decreasing order, return the total
number of distinct seconds during which the creature is poisoned.

## Example 1

```
time_series = [1, 4, 10]
duration    = 3
output      = 9      # seconds 1-3, 4-6 and 10-12
```

## Example 2

```
time_series = [2, 3, 5]
duration    = 4
output      = 7      # 2 restarts at 3, 3 restarts at 5, poisoned 2..8
```

## Constraints

- `1 <= len(time_series) <= 10^4`
- `0 <= time_series[i] <= 10^7`, non-decreasing
- `0 <= duration <= 10^7` (with `duration = 0` an attack poisons nothing)
- The answer is at most `2 * 10^7`, so it fits in a 32-bit integer.
