Given an integer array `nums`, decide whether it contains a **132 pattern**:
three indices `i < j < k` with

```
nums[i] < nums[k] < nums[j]
```

That is, a small value, then a large value, then a middle value strictly
between the two. The three indices need not be adjacent. Return `true` if
such indices exist, otherwise `false`.

## Example 1

```
nums   = [6, 2, 8, 5, 9]
output = true    # 2, 8, 5 (indices 1, 2, 3)
```

## Example 2

```
nums   = [5, 4, 4, 6, 7]
output = false
```

## Constraints

- `1 <= len(nums) <= 2 * 10^5`
- `-10^9 <= nums[i] <= 10^9`
- Aim for O(n) time.
