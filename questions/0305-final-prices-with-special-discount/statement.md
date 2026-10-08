A shop lists its items in order, and `prices[i]` is the price of item `i`.
When you buy item `i`, you get a discount equal to `prices[j]`, where `j` is
the **smallest** index with `j > i` and `prices[j] <= prices[i]`. If no such
`j` exists, there is no discount.

Return an array where entry `i` is the price you actually pay for item `i`.

## Example 1

```
prices = [9, 5, 7, 3, 4]
output = [4, 2, 4, 3, 4]
# 9 is discounted by 5, 5 by 3, 7 by 3; 3 and 4 have no smaller-or-equal
# price after them.
```

## Example 2

```
prices = [6, 6, 2, 8, 2]
output = [0, 4, 0, 6, 2]
# The first 6 is discounted by the equal 6 right after it.
```

## Constraints

- `1 <= len(prices) <= 10^5`
- `1 <= prices[i] <= 10^4`
