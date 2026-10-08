You are given an array `nums` of non-negative integers and a list of queries.
Each query is a pair `[x, m]`.

To answer a query, look only at the elements of `nums` that are **at most**
`m`. Among those, pick the one whose bitwise XOR with `x` is as large as
possible and report that XOR value. If no element of `nums` is `<= m`, the
answer to that query is `-1`.

Return the answers as an array, where `answer[i]` belongs to `queries[i]`.

## Example 1

```
nums    = [5, 1, 8, 3]
queries = [[6, 4], [2, 0], [9, 10]]
output  = [7, -1, 12]
```

Query `[6, 4]` may use `1` and `3`: `6 ^ 1 = 7`, `6 ^ 3 = 5`, so the answer is
`7`. Nothing in `nums` is `<= 0`, so the second answer is `-1`. Query
`[9, 10]` may use every element and `9 ^ 5 = 12` is the best.

## Example 2

```
nums    = [7, 7, 2]
queries = [[0, 100], [7, 6], [5, 7]]
output  = [7, 5, 7]
```

## Constraints

- `1 <= len(nums) <= 5 * 10^4`
- `1 <= len(queries) <= 5 * 10^4`
- `queries[i] = [x, m]`
- `0 <= nums[j], x, m <= 10^9`
