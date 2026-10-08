A set of `n` towns, numbered `1` to `n`, was originally linked by `n - 1`
two-way roads so that every town could reach every other town in exactly one
way (the roads formed a tree). Later, one extra road was built between two
towns that were not directly linked yet.

You are given all `n` roads in `edges`, where each entry `[a, b]` (with
`a < b`) is a road between towns `a` and `b`. Find a road whose removal turns
the network back into a tree, and return it exactly as it appears in `edges`.
If several roads would work, return the one that appears **last** in `edges`.

## Example 1

```
edges  = [[1, 2], [1, 3], [2, 3]]
output = [2, 3]     # removing any road works; [2, 3] is listed last
```

## Example 2

```
edges  = [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]
output = [1, 4]     # the loop is 1-2-3-4-1; [1, 5] is not on it
```

## Constraints

- `n == len(edges)`
- `3 <= n <= 1000`
- `1 <= a < b <= n`
- No road appears twice.
- The roads connect all `n` towns.
