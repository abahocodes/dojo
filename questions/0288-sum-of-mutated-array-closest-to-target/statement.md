You are given an integer array `arr` and an integer `target`. Pick an integer
cap `v`, then replace every element of `arr` that is **greater than** `v` with
`v` (elements at most `v` stay as they are).

Return the cap `v` that makes the sum of the resulting array as close as
possible to `target`, measured by absolute difference. If several caps give
the same smallest difference, return the **smallest** such `v`.

The cap does not have to be an element of `arr`, and it may be `0`.

## Example 1

```
arr    = [5, 2, 8]
target = 10
output = 4     # capped array [4, 2, 4] sums to exactly 10
```

## Example 2

```
arr    = [2, 3, 5]
target = 10
output = 5     # every cap >= 5 leaves the sum at 10; the smallest is 5
```

## Constraints

- `1 <= len(arr) <= 10^4`
- `1 <= arr[i] <= 10^5`
- `1 <= target <= 10^5`
