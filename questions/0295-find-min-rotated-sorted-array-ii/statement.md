An array was sorted in non-decreasing order and then rotated: some suffix of
it was moved, intact, to the front. For example, `[0, 1, 1, 5, 5, 6, 6]` might
become `[5, 6, 6, 0, 1, 1, 5]`. The rotation may also be by zero positions.
Values can repeat.

Given the rotated array `nums`, return its smallest value.

## Example 1

```
nums   = [5, 6, 6, 0, 1, 1, 5]
output = 0
```

## Example 2

```
nums   = [3, 3, 1, 3]
output = 1
```

## Constraints

- `1 <= len(nums) <= 5000`
- `-5000 <= nums[i] <= 5000`
- `nums` is a rotation of a non-decreasing array.

**Follow-up:** aim for O(log n) on typical inputs. Repeated values can force
O(n) in the worst case; explain why.
