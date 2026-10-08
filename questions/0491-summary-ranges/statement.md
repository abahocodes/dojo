You are given an array `nums` of distinct integers sorted in strictly
increasing order. Describe exactly the set of numbers in `nums` using as few
ranges as possible, where a range `[a, b]` stands for every integer from `a`
to `b` inclusive. Every number in `nums` must belong to some range, and no
range may include a number that is missing from `nums`.

Return the ranges in increasing order, each written as a string:

- `"a->b"` when the range covers more than one number (`a != b`);
- `"a"` when it covers a single number.

An empty array produces an empty list.

## Example 1

```
nums   = [0, 1, 2, 4, 5, 7]
output = ["0->2", "4->5", "7"]
```

## Example 2

```
nums   = [-3, -1, 0, 1, 5]
output = ["-3", "-1->1", "5"]
```

## Constraints

- `0 <= len(nums) <= 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`
- `nums` is sorted in strictly increasing order.
