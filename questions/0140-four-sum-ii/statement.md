You are given four integer arrays `nums1`, `nums2`, `nums3` and `nums4`,
all of the same length `n`. Count the index tuples `(i, j, k, l)`, where each
index ranges over `0 .. n - 1`, such that

```
nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0
```

Tuples are counted by their indices, so two tuples that pick equal values at
different positions are counted separately. Return the count.

## Example 1

```
nums1  = [1, -1]
nums2  = [0, 2]
nums3  = [-2, 1]
nums4  = [1, 0]
output = 3
```

The matching tuples are `(0,0,0,0)` (`1 + 0 - 2 + 1`), `(1,1,0,0)`
(`-1 + 2 - 2 + 1`) and `(1,0,1,1)` (`-1 + 0 + 1 + 0`).

## Example 2

```
nums1  = [0, 0]
nums2  = [0, 0]
nums3  = [0, 0]
nums4  = [0, 0]
output = 16  # every one of the 2^4 index tuples sums to 0
```

## Constraints

- `n == len(nums1) == len(nums2) == len(nums3) == len(nums4)`
- `1 <= n <= 200`
- `-2^28 <= nums1[i], nums2[i], nums3[i], nums4[i] <= 2^28`
