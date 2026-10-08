A row of flowers is planted in a garden bed. Flower `i` opens on day
`bloom_day[i]` and stays open from then on.

You want to pick `m` bouquets. Each bouquet needs exactly `k` flowers that
are **adjacent** in the row and have all opened, and a flower can go into at
most one bouquet.

Return the **earliest day** on which you can pick all `m` bouquets, or `-1`
if that never becomes possible.

## Example 1

```
bloom_day = [2, 9, 3, 8, 2, 1]
m         = 2
k         = 2
output    = 8
# day 3: open flowers are at 0, 2, 4, 5 -> only (4, 5) are adjacent: 1 bouquet
# day 8: open at 0, 2, 3, 4, 5 -> (2, 3) and (4, 5): 2 bouquets
```

## Example 2

```
bloom_day = [5, 5, 5, 5, 5]
m         = 3
k         = 2
output    = -1     # 3 bouquets of 2 need 6 flowers, but there are only 5
```

## Constraints

- `1 <= len(bloom_day) <= 10^5`
- `1 <= bloom_day[i] <= 10^9`
- `1 <= m <= 10^6`
- `1 <= k <= len(bloom_day)`
