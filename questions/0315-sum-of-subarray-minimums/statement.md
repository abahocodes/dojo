Given an array of positive integers `arr`, consider every contiguous,
non-empty subarray `arr[i..j]` (with `0 <= i <= j < n`). Each one has a
minimum value. Return the sum of those minimums over all `n * (n + 1) / 2`
subarrays, taken modulo `10^9 + 7`.

Subarrays are identified by their positions, so two subarrays with the same
values at different positions are both counted, and a subarray whose minimum
appears several times still contributes it only once.

## Example 1

```
arr    = [4, 1, 3]
output = 11
```

The minimums of `[4]`, `[1]`, `[3]`, `[4,1]`, `[1,3]` and `[4,1,3]` are
`4, 1, 3, 1, 1, 1`, which add up to `11`.

## Example 2

```
arr    = [2, 2, 5]
output = 15
```

`[2]`, `[2]`, `[5]`, `[2,2]`, `[2,5]`, `[2,2,5]` have minimums
`2, 2, 5, 2, 2, 2`, summing to `15`.

## Constraints

- `1 <= len(arr) <= 3 * 10^4`
- `1 <= arr[i] <= 3 * 10^4`
