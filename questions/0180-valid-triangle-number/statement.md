You are given an array `nums` of non-negative integers. Count how many ways
there are to choose three positions `i < j < k` such that the values
`nums[i]`, `nums[j]` and `nums[k]` can be the side lengths of a
**non-degenerate** triangle, that is, each value is strictly smaller than the
sum of the other two.

Triples are counted by position: equal values at different positions form
different triples.

## Example 1

```
nums   = [3, 4, 5, 6]
output = 4    # every choice of three works: (3,4,5) (3,4,6) (3,5,6) (4,5,6)
```

## Example 2

```
nums   = [0, 2, 2, 3]
output = 1    # only (2, 2, 3); a side of length 0 never forms a triangle
```

## Constraints

- `1 <= len(nums) <= 1000`
- `0 <= nums[i] <= 1000`
