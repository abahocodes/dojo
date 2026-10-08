You are given a list `arr` of **distinct** integers. Let `d` be the smallest
absolute difference between any two of its elements.

Return every pair `[a, b]` of elements with `a < b` and `b - a == d`. List the
pairs in ascending order of `a` (since the values are distinct, no two pairs
share the same `a`).

## Example 1

```
arr    = [8, 1, 5, 3]
output = [[1, 3], [3, 5]]    # d = 2
```

## Example 2

```
arr    = [20, -4, 11, 2, 27]
output = [[-4, 2]]           # d = 6; 27 - 20 = 7 is not minimal
```

## Constraints

- `2 <= len(arr) <= 10^5`
- `-10^6 <= arr[i] <= 10^6`
- All values in `arr` are distinct.
