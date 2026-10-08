For a list of numbers, its **spread** is its largest value minus its smallest
value.

Given an integer list `nums`, consider every non-empty contiguous subarray of
`nums` and add up their spreads. Return that total.

## Example 1

```
nums = [2, 5, 1]
output = 11
```

The single-element subarrays have spread 0; `[2, 5]` has 3, `[5, 1]` has 4 and
`[2, 5, 1]` has 4, so the total is `0 + 0 + 0 + 3 + 4 + 4 = 11`.

## Example 2

```
nums = [4, -1, 4]
output = 15
```

`[4, -1]`, `[-1, 4]` and `[4, -1, 4]` each have spread 5.

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^5 <= nums[i] <= 10^5`
- The answer fits in a 64-bit signed integer (it is at most about `10^15`).

**Follow-up:** an O(n^2) double loop is easy; aim for O(n).
