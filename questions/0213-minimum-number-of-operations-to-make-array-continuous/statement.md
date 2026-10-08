Call an array of length `n` **continuous** when

- all of its elements are distinct, and
- its largest element minus its smallest element equals `n - 1`.

In other words, its values are exactly the integers `x, x+1, ..., x+n-1` for
some `x`, in any order.

In one operation you may overwrite any single element of `nums` with any
integer you like. Return the minimum number of operations needed to make
`nums` continuous.

## Example 1

```
nums   = [8, 5, 6, 12, 5]
output = 2    # keep 8, 5, 6; e.g. turn 12 into 7 and the second 5 into 9
```

## Example 2

```
nums   = [3, 1, 2]
output = 0    # already continuous
```

## Constraints

- `1 <= n <= 10^5`
- `1 <= nums[i] <= 10^9`
