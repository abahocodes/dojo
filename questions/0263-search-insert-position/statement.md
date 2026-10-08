You are given an array `nums` of **distinct** integers sorted in increasing
order, and an integer `target`.

- If `target` is in `nums`, return its index.
- Otherwise return the index at which `target` would have to be inserted so
  that `nums` stays sorted. This is the number of elements smaller than
  `target`, and can be `len(nums)` when `target` is larger than everything.

Aim for `O(log n)` time.

## Example 1

```
nums   = [2, 4, 7, 10]
target = 7
output = 2
```

## Example 2

```
nums   = [2, 4, 7, 10]
target = 5
output = 2    # 5 would go between 4 and 7
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i], target <= 10^9`
- `nums` is strictly increasing.
