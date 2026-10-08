You are given `k` integer arrays, each already sorted in non-decreasing order.
Some of them may be empty, and `k` itself may be `0`.

Combine them into one array that contains every element of every input array
(duplicates included) and is itself sorted in non-decreasing order.

## Example 1

```
arrays = [[1, 4, 9], [2, 3, 10], [4, 7]]
output = [1, 2, 3, 4, 4, 7, 9, 10]
```

## Example 2

```
arrays = [[], [-5, 0], []]
output = [-5, 0]
```

## Constraints

- `0 <= k <= 10^3`
- The arrays hold at most `10^5` elements in total.
- `-10^9 <= arrays[i][j] <= 10^9`
- Each `arrays[i]` is sorted in non-decreasing order.
