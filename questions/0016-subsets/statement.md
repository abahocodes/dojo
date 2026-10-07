You are given a list `nums` of **distinct** integers. Return every possible
subset of `nums` (its power set), including the empty subset and `nums` itself.

Each subset must appear exactly once. Both the subsets and the values inside
each subset may be listed in any order.

## Example 1

```
nums   = [4, 7, 1]
output = [[], [4], [7], [1], [4, 7], [4, 1], [7, 1], [4, 7, 1]]
```

## Example 2

```
nums   = [-3]
output = [[], [-3]]
```

## Constraints

- `0 <= len(nums) <= 10`
- `-10 <= nums[i] <= 10`
- All values in `nums` are distinct.
