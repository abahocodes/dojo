You are given `nums`, a permutation of the integers `0, 1, ..., n - 1`.

- A **global inversion** is a pair of indices `i < j` with `nums[i] > nums[j]`.
- A **local inversion** is an index `i` (with `i + 1 < n`) such that
  `nums[i] > nums[i + 1]`.

Every local inversion is also a global inversion. Return `true` if the number
of global inversions equals the number of local inversions, and `false`
otherwise.

## Example 1

```
nums   = [1, 0, 2, 4, 3]
output = true    # 2 global, 2 local: (1,0) and (4,3)
```

## Example 2

```
nums   = [2, 0, 1]
output = false   # global: (2,0), (2,1); local: only (2,0)
```

## Constraints

- `1 <= n <= 10^5`
- `nums` is a permutation of `0, 1, ..., n - 1`.
