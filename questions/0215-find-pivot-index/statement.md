A **balance point** of an array `nums` is an index `i` where the sum of the
elements strictly to the left of `i` equals the sum of the elements strictly
to the right of `i`. The element at `i` itself belongs to neither side, and an
empty side sums to `0`.

Return the **smallest** balance point of `nums`, or `-1` if there is none.

## Example 1

```
nums   = [2, 7, 3, 5, 4]
output = 2    # 2 + 7 == 5 + 4
```

## Example 2

```
nums   = [1, 2, 3]
output = -1
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-1000 <= nums[i] <= 1000`
