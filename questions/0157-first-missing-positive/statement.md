Given an unsorted integer array `nums`, find the **smallest positive integer**
(1, 2, 3, ...) that does not occur anywhere in it.

Aim for a solution that runs in O(n) time and uses only O(1) extra memory
beyond the input array itself (you may rearrange `nums`).

## Example 1

```
nums   = [3, 4, -1, 1]
output = 2       # 1 is present, 2 is not
```

## Example 2

```
nums   = [8, 9, 10, 12]
output = 1       # no small values at all
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-2^31 <= nums[i] <= 2^31 - 1`
- Values may repeat.
