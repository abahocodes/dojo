You are given an integer array `nums` of length `n`. Every value lies in the
range `[1, n]`, and each value occurs either once or twice.

Return all values that occur twice, in any order. Return an empty array if
there are none.

Aim for O(n) time using only constant extra space besides the output (you
may modify `nums`).

## Example 1

```
nums   = [5, 3, 1, 3, 6, 5]
output = [3, 5]
```

## Example 2

```
nums   = [2, 1]
output = []
```

## Constraints

- `1 <= n <= 10^5`
- `1 <= nums[i] <= n`
- Each value appears at most twice.
