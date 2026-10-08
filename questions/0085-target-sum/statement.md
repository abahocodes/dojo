You are given a list of non-negative integers `nums` and an integer `target`.
Write a `+` or a `-` sign in front of **every** number, then evaluate the
resulting expression from left to right.

Return the number of different sign assignments whose expression evaluates to
exactly `target`. Two assignments are different if at least one position gets
a different sign, even when the numbers at those positions are equal (or zero).

## Example 1

```
nums   = [1, 2, 1, 3]
target = 3
output = 2      # +1 + 2 - 1 + 3  and  -1 + 2 + 1 + 3
```

## Example 2

```
nums   = [2, 0, 3]
target = 1
output = 2      # -2 + 0 + 3  and  -2 - 0 + 3
```

## Constraints

- `1 <= len(nums) <= 20`
- `0 <= nums[i] <= 1000`
- `0 <= sum(nums) <= 1000`
- `-1000 <= target <= 1000`
