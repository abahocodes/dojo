You are given an array `nums` of **distinct** integers. In one operation you
may pick any two positions and exchange their values; the positions do not
have to be adjacent.

Return the smallest number of such exchanges needed to put `nums` in
ascending order.

## Example 1

```
nums   = [4, 3, 1, 2]
output = 3   # all four values sit in one cycle of length 4
```

## Example 2

```
nums   = [10, 30, 20]
output = 1   # exchange 30 and 20
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i] <= 10^9`
- All values in `nums` are distinct.
