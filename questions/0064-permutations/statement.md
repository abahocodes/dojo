You are given a list `nums` of **distinct** integers. Return every way to
arrange all of its values in a row, that is, every permutation of `nums`.

Each permutation must appear exactly once. The permutations may be listed in
any order, but the values inside each one are, of course, in the order that
permutation describes.

## Example 1

```
nums   = [2, 7, 4]
output = [[2, 7, 4], [2, 4, 7], [7, 2, 4], [7, 4, 2], [4, 2, 7], [4, 7, 2]]
```

## Example 2

```
nums   = [-1, 0]
output = [[-1, 0], [0, -1]]
```

## Constraints

- `1 <= len(nums) <= 6`
- `-10 <= nums[i] <= 10`
- All values in `nums` are distinct.
