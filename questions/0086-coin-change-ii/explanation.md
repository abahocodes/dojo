# Approach: unbounded knapsack, coins in the outer loop

To count each multiset once, build it in a fixed order of denominations. Let
`ways[i][x]` be the number of combinations of the first `i` coin types that
sum to `x`. For the `i`-th coin `c`, either we use none of it, or we use at
least one (and then may use more):

```
ways[i][x] = ways[i-1][x] + ways[i][x - c]
```

Rolling this into one array: process the coins one by one, and for each coin
sweep `x` **upwards**, so `ways[x - c]` already includes this coin (allowing
it to be used again).

```python
def count_combinations(coins, amount):
    ways = [1] + [0] * amount
    for c in coins:
        for x in range(c, amount + 1):
            ways[x] += ways[x - c]
    return ways[amount]
```

Compare with the classic *minimum coins* problem, where the loop order doesn't
matter: here swapping the loops (amounts outside, coins inside) would count
every ordering of the same coins separately.

## Complexity

- Time: O(len(coins) × amount).
- Space: O(amount).

## Pitfalls

- Amounts in the outer loop count permutations: for `[1, 2]` and amount `3` you
  would get `3` (`1+1+1`, `1+2`, `2+1`) instead of `2`.
- Sweeping `x` downwards turns it into 0/1 knapsack (each coin at most once).
- `amount = 0` has one combination, not zero.
- Coins larger than `amount` are simply skipped by the loop range.
