A vending machine can pay out coins of the denominations listed in `coins`, and
it has an unlimited supply of each one. It needs to pay out exactly `amount`.

Return the **smallest number of coins** that add up to `amount`. If no
combination of coins hits the amount exactly, return `-1`. Paying out `0`
takes `0` coins.

## Example 1

```
coins  = [1, 5, 6, 9]
amount = 11
output = 2      # 5 + 6 (greedy 9 + 1 + 1 would use 3)
```

## Example 2

```
coins  = [4, 10]
amount = 7
output = -1     # every combination of 4s and 10s is even
```

## Constraints

- `1 <= len(coins) <= 12`
- `1 <= coins[i] <= 2^31 - 1`, all distinct
- `0 <= amount <= 10^4`
