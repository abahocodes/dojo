You are given an array `nums` of **distinct** positive integers. Count the
ordered tuples `(a, b, c, d)` such that

- `a`, `b`, `c` and `d` are all elements of `nums`,
- they are four different elements, and
- `a * b == c * d`.

Order matters: `(2, 6, 3, 4)` and `(6, 2, 3, 4)` are different tuples.
Return the count.

## Example 1

```
nums   = [1, 2, 3, 6]
output = 8   # 1 * 6 == 2 * 3, arranged in 8 orders
```

## Example 2

```
nums   = [2, 3, 4, 6, 12]
output = 16  # 2 * 6 == 3 * 4 and 2 * 12 == 4 * 6, 8 orders each
```

## Constraints

- `1 <= len(nums) <= 1000`
- `1 <= nums[i] <= 10^4`
- All values in `nums` are distinct.
- The answer fits in a 32-bit signed integer.
