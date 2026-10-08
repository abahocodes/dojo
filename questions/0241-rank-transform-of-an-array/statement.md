Given an integer array `arr`, replace every element with its **rank** and
return the resulting array. Ranks follow these rules:

- ranks are integers starting at `1`;
- a larger value gets a larger rank;
- equal values get the same rank;
- ranks are as small as possible, so the distinct values receive the ranks
  `1, 2, 3, ...` with no gaps.

## Example 1

```
arr    = [40, 10, 20, 30]
output = [4, 1, 2, 3]
```

## Example 2

```
arr    = [7, -3, 7, 7, 0, -3]
output = [3, 1, 3, 3, 2, 1]
```

## Constraints

- `0 <= len(arr) <= 10^5`
- `-2^31 <= arr[i] <= 2^31 - 1`
