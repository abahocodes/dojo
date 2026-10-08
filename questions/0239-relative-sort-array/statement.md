You are given two integer arrays, `arr1` and `arr2`. The values in `arr2` are
all different, and every one of them also appears in `arr1`.

Rearrange `arr1` so that:

- the values that appear in `arr2` come first, in the same order as they are
  listed in `arr2`, with all copies of a value placed next to each other;
- the values that do not appear in `arr2` come after them, in ascending order.

Return the rearranged array.

## Example 1

```
arr1   = [4, 1, 9, 4, 7, 2, 1, 8, 3]
arr2   = [1, 4, 2]
output = [1, 1, 4, 4, 2, 3, 7, 8, 9]
```

## Example 2

```
arr1   = [30, 5, 12, 5, 30]
arr2   = [30]
output = [30, 30, 5, 5, 12]
```

## Constraints

- `1 <= len(arr1), len(arr2) <= 1000`
- `0 <= arr1[i], arr2[i] <= 1000`
- The values of `arr2` are distinct and each appears in `arr1`.
