A cashier has an unlimited supply of coins of each denomination in `coins`
(all different). Return the number of distinct **combinations** of coins that
add up to exactly `amount`.

A combination is a multiset: only how many coins of each denomination are
used matters, not their order. So `2 + 3` and `3 + 2` count once. Paying out
`0` has exactly one combination (no coins at all). The answer is guaranteed to
fit in a signed 32-bit integer.

## Example 1

```
coins  = [2, 3, 5]
amount = 10
output = 4      # 5+5, 2+3+5, 2+2+3+3, 2+2+2+2+2
```

## Example 2

```
coins  = [3, 5]
amount = 7
output = 0
```

## Constraints

- `1 <= len(coins) <= 300`
- `1 <= coins[i] <= 5000`, all distinct
- `0 <= amount <= 5000`
