A shop sells `n` ice cream bars; bar `i` costs `costs[i]` coins. You have
`coins` coins and may buy the bars in any order, but each bar at most once.

Return the largest number of bars you can buy without spending more than
`coins` in total.

Aim for a solution that runs in time linear in `n` plus the largest price
(counting sort), rather than a comparison sort.

## Example 1

```
costs  = [3, 1, 4, 1, 5]
coins  = 7
output = 3   # buy the bars costing 1, 1 and 3 (total 5)
```

## Example 2

```
costs  = [9, 12, 10]
coins  = 8
output = 0   # every bar is too expensive
```

## Constraints

- `1 <= len(costs) <= 10^5`
- `1 <= costs[i] <= 10^5`
- `1 <= coins <= 10^8`
