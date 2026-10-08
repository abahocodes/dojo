You are given the `root` of a binary tree with `n` nodes whose values are
exactly `1, 2, ..., n` (each used once), and an array `queries`.

For each query value `q`, imagine deleting the node with value `q` together
with its entire subtree, and report the **height** of what remains: the number
of edges on the longest path from the root down to any remaining node. Each
query is independent: the tree is restored before the next one. No query names
the root.

Return the answers in the same order as `queries`.

## Example 1

```
root    = [5, 8, 9, 2, 1, 3, 7, 4, 6]
queries = [3, 2, 4, 8]
output  = [3, 2, 3, 2]
```

Removing `2` takes `4` and `6` with it, so the deepest remaining nodes are at
depth `2`. Removing `4` leaves `6`, still at depth `3`.

## Example 2

```
root    = [1, 3, 4, 2, null, 6, 5, null, null, null, null, null, 7]
queries = [4]
output  = [2]
```

## Constraints

- `2 <= n <= 10^5`
- Node values are `1..n`, all distinct.
- `1 <= queries.length <= 10^4`
- `1 <= queries[i] <= n`, and `queries[i]` is never the root's value.
  Values may repeat across queries.
