You are given a list of integers `nums` and an integer `k`. Find the `k`
distinct values that occur **most often** in `nums`.

The input guarantees that the answer is well defined: the `k`-th most common
value occurs strictly more often than any value outside the top `k`. Return the
`k` values in any order.

## Example 1

```
nums   = [7, 3, 7, 2, 3, 7, 9, 3, 7]
k      = 2
output = [7, 3]      # 7 appears 4 times, 3 appears 3 times, 2 and 9 once each
```

## Example 2

```
nums   = [-5]
k      = 1
output = [-5]
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^4 <= nums[i] <= 10^4`
- `1 <= k <=` the number of distinct values in `nums`
- The top `k` values are uniquely determined.

**Follow-up:** sorting all the distinct values by count is O(n log n). Can you
beat that?
