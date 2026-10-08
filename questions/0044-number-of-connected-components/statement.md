A network has `n` machines, labelled `0` to `n - 1`. Each entry `[a, b]` in
`edges` is a cable joining machines `a` and `b`; cables work in both
directions. Machines that can reach each other through any chain of cables form
one **group**. A machine with no cables is a group on its own.

Return how many separate groups the network has.

## Example 1

```
n     = 5
edges = [[0, 1], [1, 2], [3, 4]]
output = 2      # {0, 1, 2} and {3, 4}
```

## Example 2

```
n     = 6
edges = [[0, 1], [2, 3], [3, 4], [4, 2]]
output = 3      # {0, 1}, {2, 3, 4} and machine 5 by itself
```

## Constraints

- `1 <= n <= 2000`
- `0 <= len(edges) <= 5000`
- `0 <= a, b < n` and `a != b`
- No cable appears twice (in either direction).
