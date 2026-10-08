A rooted tree has `n` nodes labelled `0` to `n - 1`. It is described by the
array `parent`: `parent[i]` is the label of node `i`'s parent, and
`parent[0] = -1` because node `0` is the root. Any other node may be the parent
of any number of children, and labels need not increase from parent to child.

You are also given a list of `queries`, each a pair `[u, v]`. For every query,
find the **lowest common ancestor** of `u` and `v`: the deepest node that has
both `u` and `v` in its subtree. A node counts as an ancestor of itself, so the
answer for `[u, u]` is `u`, and if `u` lies above `v` the answer is `u`.

Return one answer per query, in the same order as `queries`.

## Example 1

```
parent  = [-1, 0, 0, 1, 1, 2, 4]
queries = [[3, 4], [3, 6], [5, 6], [2, 2], [6, 1]]
output  = [1, 1, 0, 2, 1]
```

```
        0
       / \
      1   2
     / \   \
    3   4   5
        |
        6
```

## Example 2

```
parent  = [-1, 0, 1, 2]
queries = [[3, 1], [2, 3], [0, 3]]
output  = [1, 2, 0]     # a path 0 - 1 - 2 - 3: the upper node is the answer
```

## Constraints

- `1 <= n <= 5 * 10^4`
- `parent[0] == -1`; for `i >= 1`, `0 <= parent[i] < n`, and the parents form
  a single tree rooted at `0`.
- `1 <= len(queries) <= 5 * 10^4`
- `0 <= u, v < n`

**Follow-up:** the tree may be a single long path, so walking up one step at a
time is too slow for many queries. Aim for `O((n + q) log n)`.
