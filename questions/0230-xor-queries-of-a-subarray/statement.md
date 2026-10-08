You are given an array of positive integers `arr` and a list of `queries`,
where `queries[i] = [l, r]` asks for the bitwise XOR of every element from
index `l` to index `r`, inclusive (0-based):
`arr[l] XOR arr[l + 1] XOR ... XOR arr[r]`.

Return an array holding the answer to each query, in the same order as the
queries.

## Example 1

```
arr     = [4, 1, 7, 2]
queries = [[0, 1], [1, 3], [2, 2], [0, 3]]
output  = [5, 4, 7, 0]   # 4^1 = 5, 1^7^2 = 4, 7, 4^1^7^2 = 0
```

## Example 2

```
arr     = [9]
queries = [[0, 0], [0, 0]]
output  = [9, 9]
```

## Constraints

- `1 <= len(arr) <= 3 * 10^4`
- `1 <= arr[i] <= 10^9`
- `1 <= len(queries) <= 3 * 10^4`
- `queries[i].length == 2`
- `0 <= l <= r < len(arr)`
