# Approach: greedy over a price histogram

Buying the cheapest bars first is optimal: if a best purchase skipped a
cheaper bar but took a pricier one, exchanging them keeps the count and lowers
the total. So the answer is the length of the longest prefix of the sorted
prices whose sum fits in `coins`.

Prices are small (at most `10^5`), so a counting sort replaces the
comparison sort. Count how many bars have each price, walk prices from low to
high, and at price `p` buy as many as you can afford: `min(count[p], coins // p)`.
If you could not buy all of them, every remaining bar costs at least `p`, so
stop.

```python
def max_ice_cream(costs, coins):
    count = [0] * (max(costs) + 1)
    for c in costs:
        count[c] += 1
    bought = 0
    for price in range(1, len(count)):
        if count[price] == 0:
            continue
        take = min(count[price], coins // price)
        bought += take
        coins -= take * price
        if take < count[price]:
            break
    return bought
```

## Complexity

- Time: O(n + C), where `C = max(costs)`.
- Space: O(C) for the histogram.

## Pitfalls

- Dynamic programming as in a knapsack: the budget reaches `10^8`, and all
  bars have the same value, so greedy is both correct and far faster.
- Buying bars one at a time inside the price loop is fine, but stopping only
  when `coins` reaches zero misses the early exit and is easy to get wrong.
- Using a comparison sort works (O(n log n)) but the exercise asks for
  counting sort.
