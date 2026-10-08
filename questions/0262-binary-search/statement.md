You are given an array `nums` whose values are **strictly increasing** and an
integer `target`. Return the index at which `target` appears in `nums`, or
`-1` if it does not appear.

Your algorithm must run in `O(log n)` time.

## Example 1

```
nums   = [-7, -2, 0, 4, 11, 30]
target = 4
output = 3
```

## Example 2

```
nums   = [1, 3, 8, 15]
target = 9
output = -1
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i], target <= 10^9`
- `nums[i] < nums[i + 1]` for every valid `i`, so all values are distinct.
