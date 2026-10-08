You are given two arrays of positive integers: `spells`, where `spells[i]` is
the strength of the `i`-th spell, and `potions`, where `potions[j]` is the
strength of the `j`-th potion. You are also given an integer `success`.

A spell and a potion form a successful pair when the product of their
strengths is at least `success`.

Return an array `result` of the same length as `spells`, where `result[i]` is
the number of potions that form a successful pair with `spells[i]`. The order
of `result` follows the order of `spells`.

## Example 1

```
spells  = [4, 1, 3]
potions = [1, 2, 3, 4, 5]
success = 8
output  = [4, 0, 3]   # 4 pairs with 2..5; 1 * 5 = 5 < 8; 3 pairs with 3, 4, 5
```

## Example 2

```
spells  = [2, 7]
potions = [6, 1, 4]
success = 14
output  = [0, 2]   # 2 * 6 = 12 < 14; 7 pairs with 6 and 4
```

## Constraints

- `1 <= len(spells), len(potions) <= 10^5`
- `1 <= spells[i], potions[j] <= 10^5`
- `1 <= success <= 10^10`
