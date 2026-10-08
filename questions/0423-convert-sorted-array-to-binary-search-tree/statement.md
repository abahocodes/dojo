You are given a list `nums` of integers in strictly increasing order. Build a
**height-balanced** binary search tree that holds exactly these values, and
return its root. Height-balanced means that at every node, the heights of the
left and right subtrees differ by at most one.

Many trees satisfy this, so build this one: the root of the tree for the
slice `nums[lo..hi]` (inclusive) is the element at index `(lo + hi) // 2` (the
left of the two middle elements when the slice has even length). Its left
subtree is built the same way from `nums[lo..mid-1]`, and its right subtree
from `nums[mid+1..hi]`.

## Example 1

```
nums   = [-10, -3, 0, 5, 9]
output = [0, -10, 5, null, -3, null, 9]
```

```
      0
     / \
  -10   5
    \    \
    -3    9
```

## Example 2

```
nums   = [1, 3]
output = [1, null, 3]    # the left middle (1) is the root
```

## Constraints

- `1 <= len(nums) <= 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is strictly increasing.
