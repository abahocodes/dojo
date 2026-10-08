You have `n` piles of candy; pile `i` holds `candies[i]` candies. You want to
hand candy to `k` children so that **every child receives exactly the same
number of candies**, and each child's share comes from a **single pile**.

You may break any pile into as many smaller piles as you like (for example a
pile of 7 can become piles of 3, 3 and 1), but you can never combine candy from
two different piles. Some candy may be left over.

Return the largest number of candies each child can receive. If it is
impossible to give every child even one candy, return `0`.

## Example 1

```
candies = [6, 9, 4]
k       = 5
output  = 3    # size 3: 2 + 3 + 1 = 6 shares; size 4: 1 + 2 + 1 = 4 shares, too few
```

## Example 2

```
candies = [3, 1]
k       = 7
output  = 0    # only 4 candies exist in total, so 7 children cannot each get one
```

## Constraints

- `1 <= n <= 10^5`
- `1 <= candies[i] <= 10^7`
- `1 <= k <= 10^12`
