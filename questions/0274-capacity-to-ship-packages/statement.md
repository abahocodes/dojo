Packages wait on a dock in a fixed line; package `i` weighs `weights[i]`. A
ship makes one trip per day. On each trip it carries some of the next
packages in line (a contiguous run, taken in order) whose total weight does
not exceed the ship's **capacity**. Packages can't be reordered, skipped or
split across days.

Return the **smallest integer capacity** that lets the ship deliver every
package in at most `days` days.

## Example 1

```
weights = [3, 2, 2, 4, 1, 4]
days    = 3
output  = 6
# day 1: 3 + 2 = 5    day 2: 2 + 4 = 6    day 3: 1 + 4 = 5
# with capacity 5 at least four days are needed
```

## Example 2

```
weights = [1, 2, 3, 1, 1]
days    = 4
output  = 3
# day 1: 1 + 2    day 2: 3    day 3: 1 + 1   (only three days needed)
```

## Constraints

- `1 <= days <= len(weights) <= 5 * 10^4`
- `1 <= weights[i] <= 500`
