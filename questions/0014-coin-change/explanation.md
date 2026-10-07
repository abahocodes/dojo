# Approach: bottom-up DP over amounts

Let `best[x]` be the fewest coins that sum to `x`. An optimal payout of `x > 0`
ends with some coin `c`, and what's left must be an optimal payout of `x - c`:

```
best[0] = 0
best[x] = 1 + min(best[x - c] for c in coins if c <= x)
```

Fill the table for `x = 1..amount`. Use `amount + 1` as "infinity", since no
real answer can need more than `amount` coins (the smallest coin is at least 1).

```python
def coin_change(coins, amount):
    INF = amount + 1
    best = [0] + [INF] * amount
    for total in range(1, amount + 1):
        for coin in coins:
            if coin <= total and best[total - coin] + 1 < best[total]:
                best[total] = best[total - coin] + 1
    return best[amount] if best[amount] != INF else -1
```

**Alternative:** breadth-first search from `0`, where every coin is an edge
`x -> x + c`. The first level at which you reach `amount` is the answer. It's
the same worst-case cost, but it stops early when the answer is small.

## Complexity

- Time: O(amount × len(coins)).
- Space: O(amount).

## Pitfalls

- Greedy (take the largest coin first) is wrong for arbitrary denominations,
  for example `[1, 5, 6, 9]` with amount `11`.
- Plain recursion without memoisation branches on every coin at every level
  and is exponential.
- Using `float("inf")` works in Python, but in fixed-width languages
  `INT_MAX + 1` overflows. A sentinel of `amount + 1` avoids that.
- `amount = 0` must return `0`, not `-1`.
- Coins larger than `amount` are allowed and simply never used.
