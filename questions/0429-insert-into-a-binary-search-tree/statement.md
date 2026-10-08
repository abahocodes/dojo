You are given the `root` of a binary search tree and an integer `val` that does
not appear in the tree. Insert `val` as a **new leaf**: start at the root and
move left when `val` is smaller than the current node's value or right when it
is larger, until you reach a missing child; attach the new node there. Do not
rearrange any existing nodes.

Return the root of the resulting tree. If the tree was empty, the new node is
the root.

## Example 1

```
root   = [8, 3, 10, 1, 6, null, 14]
val    = 5
output = [8, 3, 10, 1, 6, null, 14, null, null, 5]
```

```
        8                    8
      /   \                /   \
     3     10      ->      3     10
    / \      \            / \      \
   1   6      14         1   6      14
                            /
                           5
```

## Example 2

```
root   = []
val    = 4
output = [4]
```

## Constraints

- The tree has between `0` and `10^4` nodes.
- `-10^8 <= node.val, val <= 10^8`
- All values in the tree are distinct, and `val` is not among them.
- Trees are given (and returned) in level order; `null` marks a missing child.
