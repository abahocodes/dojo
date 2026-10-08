For every position `i` of the array `nums`, compare two totals:

- the **left total**: the sum of all elements strictly before index `i`
  (`0` when `i` is the first index), and
- the **right total**: the sum of all elements strictly after index `i`
  (`0` when `i` is the last index).

Return an array `answer` of the same length where `answer[i]` is the absolute
difference between the left total and the right total at `i`.

## Example 1

```
nums   = [4, 9, 1, 3]
output = [13, 0, 10, 14]
# i = 0: |0 - 13| = 13      i = 1: |4 - 4| = 0
# i = 2: |13 - 3| = 10      i = 3: |14 - 0| = 14
```

## Example 2

```
nums   = [7]
output = [0]     # nothing on either side
```

## Constraints

- `1 <= len(nums) <= 1000`
- `1 <= nums[i] <= 10^5`
