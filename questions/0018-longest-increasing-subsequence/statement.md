Given an integer array `nums`, return the length of the **longest strictly increasing subsequence**. A subsequence is derived by deleting zero or more elements without changing the order of the remaining elements; consecutive elements in the subsequence must satisfy `a[i] < a[i+1]`.

## Example 1

```
nums = [10, 9, 2, 5, 3, 7, 101, 18]
output = 4
# One valid LIS is [2, 3, 7, 101] (or [2, 5, 7, 101], [2, 3, 7, 18], …)
```

## Example 2

```
nums = [0, 1, 0, 3, 2, 3]
output = 4
# One valid LIS is [0, 1, 2, 3]
```

## Constraints

- `0 <= len(nums) <= 2500`
- `-10^4 <= nums[i] <= 10^4`

**Follow-up:** Can you solve it in O(n log n) time? The O(n²) dynamic-programming approach passes within the given constraints, but the patience-sorting / binary-search method is the expected optimal solution.
