A row of cards lies on the table; card `i` is worth `card_points[i]` points.
You must take exactly `k` cards, one at a time, and each card you take must be
the current **leftmost or rightmost** card still in the row.

Return the largest total number of points you can collect.

## Example 1

```
card_points = [2, 9, 1, 1, 1, 8, 7]
k           = 3
output      = 18   # take 2 and 9 from the left, then 7 from the right
```

## Example 2

```
card_points = [5, 5, 5]
k           = 3
output      = 15   # all cards must be taken
```

## Constraints

- `1 <= k <= len(card_points) <= 10^5`
- `1 <= card_points[i] <= 10^4`
