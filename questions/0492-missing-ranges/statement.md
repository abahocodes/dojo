You are given a range of integers `[lower, upper]` (both ends included) and a
sorted array `nums` of distinct integers, all of which lie inside that range.

Some integers of the range do not appear in `nums`. Group them into **maximal
blocks of consecutive missing integers** and return each block as a pair
`[a, b]`, meaning every integer from `a` to `b` is missing (a single missing
integer `x` is reported as `[x, x]`).

Return the blocks in increasing order. If nothing is missing, return an empty
list.

## Example 1

```
nums   = [2, 3, 7, 12]
lower  = 0
upper  = 15
output = [[0, 1], [4, 6], [8, 11], [13, 15]]
```

## Example 2

```
nums   = [-4, -3, -2]
lower  = -4
upper  = -2
output = []      # every integer in the range is present
```

## Constraints

- `0 <= len(nums) <= 10^4`
- `-10^9 <= lower <= upper <= 10^9`
- `nums` is strictly increasing and every value satisfies `lower <= nums[i] <= upper`.
