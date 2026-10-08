Take a complete binary tree with `2^n - 1` nodes, labelled like a binary heap:
the root is `1`, and the children of node `v` are `2v` (left) and `2v + 1`
(right).

Each query `[a, b]` asks: if you add one extra edge directly between `a` and
`b`, the graph gains exactly one cycle. How many edges does that cycle have?
Every query is answered on the original tree (the extra edge is removed
again afterwards).

Return the cycle lengths, one per query, in order.

## Example 1

```
n       = 3
queries = [[5, 3], [4, 7], [2, 3]]
output  = [4, 5, 3]
```

For `[5, 3]`: the tree path is `5 - 2 - 1 - 3` (3 edges), plus the new edge
gives a cycle of 4.

## Example 2

```
n       = 2
queries = [[1, 2]]
output  = [2]
```

The extra edge doubles the existing edge `1 - 2`, forming a cycle of length 2.

## Constraints

- `2 <= n <= 30`
- `1 <= queries.length <= 10^5`
- `1 <= a, b <= 2^n - 1` and `a != b`
