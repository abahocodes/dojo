You are given `n` nodes labelled `0` to `n - 1` and a list of undirected
`edges`, where each `[a, b]` joins node `a` and node `b`.

Return `true` if these edges form a single **tree** that covers all `n` nodes,
meaning every node can reach every other node and there is no cycle.
Otherwise return `false`.

## Example 1

```
n     = 5
edges = [[0, 1], [0, 2], [0, 3], [1, 4]]
output = true
```

## Example 2

```
n     = 5
edges = [[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]]
output = false     # 1 - 2 - 3 - 1 is a cycle
```

## Example 3

```
n     = 4
edges = [[0, 1], [2, 3]]
output = false     # {0, 1} and {2, 3} are not connected
```

## Constraints

- `1 <= n <= 2000`
- `0 <= len(edges) <= 5000`
- `0 <= a, b < n` and `a != b`
- No edge is listed twice (in either direction).
