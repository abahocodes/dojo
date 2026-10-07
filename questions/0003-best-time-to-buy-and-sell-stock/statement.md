`prices[d]` is the price of a share on day `d`. You may make **at most one
trade**: buy one share on some day, then sell it on a **strictly later** day.

Return the largest profit you can make. If no trade makes money, return `0`
(you simply don't trade).

## Example 1

```
prices = [9, 4, 6, 2, 8, 5]
output = 6        # buy at 2 (day 3), sell at 8 (day 4)
```

## Example 2

```
prices = [10, 8, 5, 5, 1]
output = 0        # prices never rise, so don't trade
```

## Constraints

- `1 <= len(prices) <= 10^5`
- `0 <= prices[d] <= 10^4`
