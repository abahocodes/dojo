Every value in the list `nums` shows up exactly **twice**, except for one value
that shows up only **once**. Find that lonely value.

Aim for a solution that walks the list once and uses only a constant amount of
extra memory.

## Example 1

```
nums   = [7, 3, 7]
output = 3
```

## Example 2

```
nums   = [9, -2, 5, -2, 9]
output = 5        # 9 and -2 both have a twin
```

## Constraints

- `1 <= len(nums) <= 3 * 10^4`, and `len(nums)` is odd
- `-3 * 10^4 <= nums[i] <= 3 * 10^4`
- Exactly one value appears once; every other value appears exactly twice.
