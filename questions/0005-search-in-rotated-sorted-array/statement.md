A list of **distinct** integers was sorted in increasing order and then
**rotated**: some prefix was cut off and moved to the end (possibly an empty
prefix, so the list may not be rotated at all). For example, `[2, 5, 7, 11, 13]`
could become `[11, 13, 2, 5, 7]`.

Given the rotated list `nums` and an integer `target`, return the index of
`target` in `nums`, or `-1` if it does not appear.

Aim for O(log n) time.

## Example 1

```
nums   = [11, 13, 2, 5, 7]
target = 5
output = 3
```

## Example 2

```
nums   = [30, 40, 50, 10, 20]
target = 35
output = -1
```

## Constraints

- `1 <= len(nums) <= 10^5`
- `-10^9 <= nums[i], target <= 10^9`
- All values in `nums` are distinct.
- `nums` is an increasing list rotated by some amount.
