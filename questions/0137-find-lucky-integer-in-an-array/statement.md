Call a value in the array `arr` **lucky** if the number of times it occurs in
`arr` is equal to the value itself. For example, `3` is lucky if `arr`
contains exactly three `3`s.

Return the largest lucky value in `arr`, or `-1` if no value is lucky.

## Example 1

```
arr    = [4, 1, 4, 3, 4, 3, 4, 3, 2]
output = 4    # 4 occurs 4 times, 3 occurs 3 times, 1 occurs once; 4 is largest
```

## Example 2

```
arr    = [2, 5, 5, 3, 3]
output = -1   # 2 occurs once, 5 twice, 3 twice
```

## Constraints

- `1 <= len(arr) <= 500`
- `1 <= arr[i] <= 500`
