Given an integer array `nums`, return its elements reordered by how often
each value occurs:

- values that occur fewer times come first;
- when two values occur the same number of times, the **larger** value comes
  first.

Every copy of a value is kept, so all copies of a value end up next to each
other.

## Example 1

```
nums   = [4, 4, 1, 3, 3, 3, 2]
output = [2, 1, 4, 4, 3, 3, 3]   # 2 and 1 occur once (2 is larger),
                                 # 4 occurs twice, 3 three times
```

## Example 2

```
nums   = [-1, 5, -1, 5, 0]
output = [0, 5, 5, -1, -1]
```

## Constraints

- `1 <= len(nums) <= 100`
- `-100 <= nums[i] <= 100`
