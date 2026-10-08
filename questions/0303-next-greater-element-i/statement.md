You are given two arrays of **distinct** integers, `nums1` and `nums2`, where
every value of `nums1` also appears in `nums2`.

For a value `x`, its **next greater element** is the first value to the right
of `x`'s position in `nums2` that is strictly greater than `x`. If no such
value exists, it is `-1`.

Return an array whose `i`-th entry is the next greater element of `nums1[i]`,
in the same order as `nums1`.

## Example 1

```
nums1  = [3, 1, 6]
nums2  = [1, 3, 5, 6, 2]
output = [5, 3, -1]
# right of 3: 5 is the first larger value
# right of 1: 3
# right of 6: only 2 remains, so -1
```

## Example 2

```
nums1  = [2, 9]
nums2  = [9, 2, 4]
output = [4, -1]
```

## Constraints

- `1 <= len(nums1) <= len(nums2) <= 10^4`
- `0 <= nums1[i], nums2[i] <= 10^4`
- All values in `nums1` are distinct, and so are all values in `nums2`.
- Every value of `nums1` appears in `nums2`.
