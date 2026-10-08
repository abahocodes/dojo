You are given an array of positive integers `nums` and a positive integer `x`.
In one operation you take either the **first** or the **last** element that is
still in the array, remove it, and subtract its value from `x`.

Return the smallest number of operations that brings `x` to **exactly** `0`.
If no sequence of operations does that, return `-1`.

## Example 1

```
nums   = [3, 1, 4, 2, 2]
x      = 6
output = 3    # take 3 and 1 from the front and 2 from the back
```

## Example 2

```
nums   = [5, 6, 7]
x      = 4
output = -1   # every element is already larger than 4
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^4`
- `1 <= x <= 10^9`
