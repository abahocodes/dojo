You are given the `root` of a binary search tree with at least two nodes and
all values distinct. Return the smallest absolute difference between the values
of any two different nodes.

## Example 1

```
root   = [4, 2, 6, 1, 3]
output = 1       # for example |2 - 1|
```

## Example 2

```
root   = [50, 20, 90, null, 35, 70]
output = 15      # |35 - 20| or |50 - 35|
```

## Constraints

- The tree has between `2` and `10^4` nodes.
- `0 <= node.val <= 10^5`, and all values are distinct.
