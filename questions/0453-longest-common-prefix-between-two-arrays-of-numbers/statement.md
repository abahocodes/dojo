You are given two arrays of positive integers, `arr1` and `arr2`. For a pair
`(x, y)` with `x` from `arr1` and `y` from `arr2`, write both numbers in
decimal (no leading zeros) and measure the length of the longest string that
both decimal strings start with.

Return the largest such length over all pairs. If no pair shares even a first
digit, return `0`.

## Example 1

```
arr1   = [123, 45, 1289]
arr2   = [1287, 450]
output = 3       # 1289 and 1287 both start with "128"
```

## Example 2

```
arr1   = [7, 8]
arr2   = [91]
output = 0       # no pair has the same first digit
```

## Constraints

- `1 <= len(arr1), len(arr2) <= 5 * 10^4`
- `1 <= arr1[i], arr2[i] <= 10^8`
