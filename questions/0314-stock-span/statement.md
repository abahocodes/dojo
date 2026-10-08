You are given the closing price of a stock on each of `n` consecutive days,
`prices[0]` through `prices[n - 1]`.

The **span** of day `i` is the length of the longest run of consecutive days
that ends on day `i` (and includes it) during which every price was less than
or equal to `prices[i]`. In other words, starting at day `i`, count how many
days you can step backwards, including day `i` itself, before you hit a day
with a strictly higher price or run out of days.

Return the span of every day, in day order.

## Example 1

```
prices = [90, 70, 60, 75, 60, 80, 95]
output = [1, 1, 1, 3, 1, 5, 7]
```

Day 3 (price 75) covers itself and the 70 and 60 before it; the 90 stops it.

## Example 2

```
prices = [5, 5, 5]
output = [1, 2, 3]   # equal prices extend the span
```

## Constraints

- `1 <= len(prices) <= 10^5`
- `1 <= prices[i] <= 10^5`
