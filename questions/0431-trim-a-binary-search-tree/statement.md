You are given the `root` of a binary search tree with distinct values and two
integers `low <= high`. Remove every node whose value is outside the range
`[low, high]`, keeping the remaining nodes in the same relative arrangement: if
a surviving node was a descendant of another surviving node before trimming, it
must still be one afterward, on the same side. The result is a valid BST and is
uniquely determined. Return its root (an empty tree if nothing survives).

## Example 1

```
root   = [6, 3, 9, 1, 4, 8, 11]
low    = 3
high   = 8
output = [6, 3, 8, null, 4]
```

```
         6                  6
       /   \              /   \
      3     9     ->     3     8
     / \   / \            \
    1   4 8   11           4
```

`1`, `9` and `11` are dropped; `8` moves up to take `9`'s place.

## Example 2

```
root   = [10, 4, 15, 2, 7, null, 20, null, null, 5]
low    = 5
high   = 16
output = [10, 7, 15, 5]
```

## Constraints

- The tree has between `1` and `10^4` nodes.
- `0 <= node.val <= 10^4`, and all values are distinct.
- `0 <= low <= high <= 10^4`
- Trees are given (and returned) in level order; `null` marks a missing child.
