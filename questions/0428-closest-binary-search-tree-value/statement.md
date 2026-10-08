You are given the `root` of a binary search tree (every value in a node's left
subtree is smaller than the node's value, every value in its right subtree is
larger) and a real number `target`. Return the value stored in the tree whose
distance to `target` is smallest. If two values are equally close, return the
**smaller** of the two.

Try to use the ordering of the tree so that you only follow a single path from
the root downward instead of visiting every node.

## Example 1

```
root   = [8, 4, 12, 2, 6, 10, 14]
target = 9.5
output = 10
```

```
        8
      /   \
     4     12
    / \   /  \
   2   6 10  14
```

`10` is 0.5 away from the target; the next best value, `8`, is 1.5 away.

## Example 2

```
root   = [7, 3, 11]
target = 5.0
output = 3
```

Both `3` and `7` are exactly 2 away from `5.0`; the tie goes to the smaller
value.

## Constraints

- The tree has between `1` and `10^4` nodes.
- `0 <= node.val <= 10^9`, and all values are distinct.
- `-10^9 <= target <= 10^9`
- Trees are given (and returned) in level order; `null` marks a missing child.
