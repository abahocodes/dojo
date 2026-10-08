You are given `k` lists of integers, each sorted in non-decreasing order.
Find the smallest range `[a, b]` (with `a <= b`) that contains at least one
number from every list. A number `x` is in the range when `a <= x <= b`.

Range `[a, b]` is **smaller** than range `[c, d]` when `b - a < d - c`, or when
the widths are equal and `a < c`. Return the smallest range as `[a, b]`.

## Example 1

```
nums   = [[1, 9, 20], [4, 12, 30], [10, 11, 25]]
output = [9, 12]
```

`9` comes from the first list, `12` from the second and `10` or `11` from the
third; no range of width less than 3 covers all three lists.

## Example 2

```
nums   = [[5, 6], [5, 6], [5, 6]]
output = [5, 5]
```

Width 0 is possible at 5 and at 6; the tie goes to the smaller start.

## Constraints

- `1 <= len(nums) <= 3500`
- `1 <= len(nums[i]) <= 50`
- `-10^5 <= nums[i][j] <= 10^5`
- Each `nums[i]` is sorted in non-decreasing order.
