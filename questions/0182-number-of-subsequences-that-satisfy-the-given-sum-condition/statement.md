You are given an integer array `nums` and an integer `target`. A
**subsequence** is any non-empty set of positions of `nums`; its minimum and
maximum are the smallest and largest values at those positions.

Count the subsequences whose minimum plus maximum is **at most** `target`.
Subsequences are distinguished by their positions, so equal values at
different positions give different subsequences. A subsequence of one element
uses that element as both its minimum and its maximum.

The answer can be huge: return it modulo `10^9 + 7`.

## Example 1

```
nums   = [2, 4, 5, 9]
target = 8
output = 5    # [2] [2,4] [2,5] [2,4,5] [4]
```

## Example 2

```
nums   = [1, 1, 1]
target = 2
output = 7    # every non-empty choice of the three positions
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `1 <= nums[i] <= 10^6`
- `1 <= target <= 10^6`
