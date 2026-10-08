You are given an array `arr` sorted in non-decreasing order, an integer `k`
and a target integer `x`. Return the `k` elements of `arr` that are closest to
`x`, listed in ascending order.

Element `a` is closer to `x` than element `b` when `|a - x| < |b - x|`. When
the distances are equal, the smaller element counts as closer. The array may
contain duplicates; each occurrence is a separate element.

## Example 1

```
arr    = [1, 3, 5, 7, 9]
k      = 3
x      = 6
output = [3, 5, 7]   # 5 and 7 are at distance 1; 3 and 9 tie at 3, and 3 is smaller
```

## Example 2

```
arr    = [2, 4, 4, 10]
k      = 2
x      = 20
output = [4, 10]
```

## Constraints

- `1 <= k <= len(arr) <= 10^5`
- `-10^4 <= arr[i], x <= 10^4`
- `arr` is sorted in non-decreasing order
