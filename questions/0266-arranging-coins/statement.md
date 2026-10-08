You have `n` coins and want to lay them out as a staircase: row `1` holds
exactly `1` coin, row `2` holds exactly `2` coins, and in general row `i`
holds exactly `i` coins. You fill the rows in order, and the last row may end
up incomplete.

Return the number of **complete** rows.

## Example 1

```
n      = 8
output = 3    # rows of 1, 2 and 3 coins use 6; the 2 left over
              # cannot finish row 4
```

## Example 2

```
n      = 10
output = 4    # 1 + 2 + 3 + 4 = 10 exactly
```

## Constraints

- `1 <= n <= 2^52`
- The answer is below `10^8`.
