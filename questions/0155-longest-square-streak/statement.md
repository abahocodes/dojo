A **square streak** in an integer array `nums` is a subsequence (elements
picked from any positions, at most once each) that satisfies both:

- it has length at least 2, and
- once sorted in ascending order, every element except the first is the
  square of the element before it.

Return the length of the longest square streak in `nums`, or `-1` if there
is none.

## Example 1

```
nums   = [16, 3, 2, 9, 4, 81, 5]
output = 3       # [3, 9, 81] or [2, 4, 16]
```

## Example 2

```
nums   = [7, 3, 10, 6]
output = -1
```

## Constraints

- `2 <= len(nums) <= 10^5`
- `2 <= nums[i] <= 10^5`
