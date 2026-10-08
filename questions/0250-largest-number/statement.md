You are given a list `nums` of non-negative integers. Write the numbers one
after another, in some order, to form a single decimal number. Return the
largest number that can be formed this way, as a string.

The result must not have leading zeros: if the largest arrangement consists
only of zeros, return `"0"`.

## Example 1

```
nums   = [12, 7, 5]
output = "7512"
```

## Example 2

```
nums   = [40, 4, 45, 9, 0]
output = "9454400"    # 4 before 40, since "440" > "404"
```

## Constraints

- `1 <= len(nums) <= 100`
- `0 <= nums[i] <= 10^9`
