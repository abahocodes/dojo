`prices[i]` is the price of a share on day `i`. You may trade as many times
as you like, under these rules:

- You hold at most one share at a time: you must sell before you buy again.
- Each day you do at most one thing: buy, sell, or nothing.
- After a sale you must rest for one day: if you sell on day `i`, you cannot
  buy on day `i + 1` (buying on day `i + 2` is fine).

Return the largest total profit you can make. Doing nothing at all is
allowed, so the answer is never negative. Holding a share at the end earns
nothing.

## Example 1

```
prices = [1, 2, 3, 0, 2]
output = 3
```

Buy on day 0 (price 1) and sell on day 1 (+1), rest on day 2 as required,
buy on day 3 (price 0) and sell on day 4 (+2). Total `3`. Holding from day 0
to day 2 earns only `2`, and the cooldown then forbids buying on day 3.

## Example 2

```
prices = [6, 1, 3, 2, 4, 7]
output = 6
```

Buying at `1` and selling at `7` earns `6`. Splitting into `1 → 3` and
`2 → 7` is not possible, because day 3 is the cooldown after the sale on
day 2; `1 → 3` then `4 → 7` makes only `5`.

## Constraints

- `1 <= len(prices) <= 5000`
- `0 <= prices[i] <= 1000`
