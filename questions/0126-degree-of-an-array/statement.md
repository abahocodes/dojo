The **degree** of an array is the largest number of times any single value
occurs in it. Given a non-empty integer array `nums`, find the shortest
contiguous subarray that has the **same degree** as the whole of `nums`, and
return its length.

## Example 1

```
nums   = [7, 3, 3, 5, 7]
output = 2       # degree 2, reached by 7 and 3; [3, 3] is the shortest
                 # subarray with two equal values
```

## Example 2

```
nums   = [6, 5, 5, 6, 6, 5]
output = 5       # degree 3, reached by both 5 and 6; [6, 5, 5, 6, 6]
                 # and [5, 5, 6, 6, 5] both have length 5
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `0 <= nums[i] < 5 * 10^4`
