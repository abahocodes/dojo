You are given a non-empty list of integers `nums`. Among all **contiguous,
non-empty** stretches of `nums`, find the one whose elements have the largest
product, and return that product.

## Example 1

```
nums   = [2, -5, -2, 4, 0, 3]
output = 80     # 2 * (-5) * (-2) * 4
```

## Example 2

```
nums   = [-3, 0, -1]
output = 0      # [0] beats every stretch with a negative product
```

## Constraints

- `1 <= len(nums) <= 2 * 10^4`
- `-10 <= nums[i] <= 10`
- The product of every contiguous stretch fits in a 32-bit signed integer.
