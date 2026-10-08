You are given a list of integers `nums`. Look at the **distinct** values it
contains, ignoring repeats. If there are at least three distinct values,
return the third largest of them. Otherwise return the largest value.

## Example 1

```
nums   = [4, 9, 1, 7]
output = 4       # distinct values in decreasing order: 9, 7, 4, 1
```

## Example 2

```
nums   = [5, 5, 3, 3, 8]
output = 3       # distinct values: 8, 5, 3; duplicates do not count twice
```

## Example 3

```
nums   = [6, 2]
output = 6       # only two distinct values, so return the largest
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`
