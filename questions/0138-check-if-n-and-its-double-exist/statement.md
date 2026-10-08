Given an integer array `arr`, decide whether some element is exactly twice
another element at a **different** position. That is, return `true` if there
are indices `i != j` with `arr[i] == 2 * arr[j]`, and `false` otherwise.

Note that `0` is twice `0`, so two zeros at different positions count, but a
single zero does not.

## Example 1

```
arr    = [7, 3, 11, 6]
output = true    # 6 == 2 * 3
```

## Example 2

```
arr    = [4, -3, 9, 0, 5]
output = false   # the lone 0 cannot pair with itself
```

## Constraints

- `2 <= len(arr) <= 10^4`
- `-10^5 <= arr[i] <= 10^5`
