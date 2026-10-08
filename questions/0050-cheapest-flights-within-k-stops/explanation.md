# Approach: Bellman-Ford with a fixed number of rounds

At most `k` stops means at most `k + 1` flights. Let `cost_i[c]` be the
cheapest price to reach city `c` from `src` using at most `i` flights. Then
`cost_0` is `0` at `src` and infinite elsewhere, and

```
cost_{i+1}[to] = min(cost_i[to], min over flights (from, to, p) of cost_i[from] + p)
```

That is one Bellman-Ford round, where the new values are computed only from
the previous round's values. Run exactly `k + 1` rounds.

```python
def find_cheapest_price(n, flights, src, dst, k):
    inf = float("inf")
    cost = [inf] * n
    cost[src] = 0
    for _ in range(k + 1):
        nxt = cost[:]
        for u, v, price in flights:
            if cost[u] + price < nxt[v]:
                nxt[v] = cost[u] + price
        cost = nxt
    return -1 if cost[dst] == inf else cost[dst]
```

**Alternative:** Dijkstra over states `(city, flights used)`, discarding states
that would exceed `k + 1` flights. It can stop early when `dst` is popped, but
the state space is `n · (k + 2)`, and the bookkeeping is easier to get wrong.

## Complexity

- Time: O((k + 1) · F), where F is the number of flights.
- Space: O(n) for the two price arrays.

## Pitfalls

- Updating the array in place. A single round could then chain several
  flights together (`0 -> 1` then `1 -> 2` in the same pass) and break the
  stop limit. Always read from the previous round.
- Off-by-one: `k` counts *stops*, so there are `k + 1` rounds, not `k`.
- Plain Dijkstra with a global "visited" set. The cheapest way into a city may
  use too many flights, and marking it visited then hides a pricier route with
  fewer flights that is the only one that fits.
