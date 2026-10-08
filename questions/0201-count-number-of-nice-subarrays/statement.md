Call a contiguous, non-empty subarray of `nums` **nice** if exactly `k` of its
elements are odd. Return how many nice subarrays `nums` has.

## Example 1

```
nums   = [3, 4, 6, 5, 7]
k      = 2
output = 4    # [3,4,6,5] [4,6,5,7] [6,5,7] [5,7]
```

## Example 2

```
nums   = [2, 8, 4]
k      = 1
output = 0    # there are no odd numbers at all
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `1 <= nums[i] <= 10^5`
- `1 <= k <= len(nums)`
