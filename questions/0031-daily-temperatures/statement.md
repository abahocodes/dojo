`temperatures[i]` is the reading taken on day `i`. For each day, find how
many days you have to wait after it until a day with a **strictly higher**
reading. If no later day is warmer, the wait for that day is `0`.

Return the list of waits, one per day.

## Example 1

```
temperatures = [70, 68, 72, 71, 69, 75]
output       = [2, 1, 3, 2, 1, 0]
```

## Example 2

```
temperatures = [50, 50, 49]
output       = [0, 0, 0]     # equal is not warmer
```

## Constraints

- `1 <= len(temperatures) <= 10^5`
- `30 <= temperatures[i] <= 100`
