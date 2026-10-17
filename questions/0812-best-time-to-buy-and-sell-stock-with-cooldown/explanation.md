# Approach: a three-state machine, one day at a time

At the end of any day you are in exactly one of three situations:

- **hold**: you own a share;
- **sold**: you sold today, so tomorrow you must rest;
- **rest**: you own nothing and are free to buy tomorrow.

Track the best profit for each state. For the next price `p`:

- `hold' = max(hold, rest - p)`: keep the share, or buy today. Buying is only
  allowed from `rest`, never straight after a sale. That is the cooldown.
- `sold' = hold + p`: sell the share you held.
- `rest' = max(rest, sold)`: stay idle, or yesterday's sale finishes its
  cooldown.

Compute all three from the previous day's values, then move on. At the end,
holding a share is never better than not having bought it, so the answer is
`max(sold, rest)`.

```python
def max_profit(prices):
    hold, sold, rest = float("-inf"), 0, 0
    for p in prices:
        hold, sold, rest = max(hold, rest - p), hold + p, max(rest, sold)
    return max(sold, rest)
```

## Complexity

- Time: O(n), one pass with constant work per day.
- Space: O(1).

## Pitfalls

- Update the states **simultaneously**. Overwriting `sold` before computing
  `rest` (or `hold` before `sold`) lets one day do two actions. Use tuple
  assignment or a temporary.
- `hold` must start as "impossible" (minus infinity, or a very negative
  number) so you can't sell a share you never bought. In fixed-width
  languages pick something like `INT_MIN / 2` so `hold + p` can't overflow.
- Buying from `sold` instead of `rest` silently drops the cooldown and
  returns the answer to the unlimited-trades problem.
- An O(n²) DP over "last sell day" also works and fits here, but the state
  machine is simpler and linear.
