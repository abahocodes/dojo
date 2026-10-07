# Approach: track the cheapest price so far

If you sell on day `d`, the best you can do is to have bought at the minimum
price among days `0..d-1`. So scan left to right, remembering the running
minimum, and at each day consider selling there.

```python
def max_profit(prices):
    lowest = prices[0]
    best = 0
    for p in prices:
        best = max(best, p - lowest)
        lowest = min(lowest, p)
    return best
```

Comparing `p - lowest` before updating `lowest` keeps the buy strictly earlier
(or on the same day, which yields 0 and is harmless).

## Complexity

- Time: O(n) — single pass.
- Space: O(1).

## Pitfalls

- Taking `max(prices) - min(prices)` ignores order: the max may come before the min.
- Returning a negative number when prices only fall — the answer floors at 0.
- An equivalent view: this is the maximum subarray sum (Kadane) over the daily
  price differences.
