You are given an array `numbers` sorted in **non-decreasing** order and an
integer `target`. Exactly one pair of different positions holds two values
whose sum is `target`.

Return the positions of that pair as a two-element array `[i, j]`, where the
positions are **1-based** (the first element is position 1) and `i < j`.

Your solution should use only a constant amount of extra memory besides the
returned array.

## Example 1

```
numbers = [1, 4, 6, 9, 13]
target  = 15
output  = [3, 4]   # 6 + 9 = 15
```

## Example 2

```
numbers = [-7, -2, 0, 3]
target  = -9
output  = [1, 2]   # -7 + -2 = -9
```

## Constraints

- `2 <= len(numbers) <= 3 * 10^4`
- `-1000 <= numbers[k] <= 1000`
- `numbers` is sorted in non-decreasing order.
- Exactly one pair of positions `i < j` satisfies `numbers[i] + numbers[j] == target`.
