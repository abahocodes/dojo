You are given the `root` of a binary tree that used to be a valid binary search
tree with distinct values, until the values of **exactly two** nodes were
swapped by mistake. Swap those two values back so that the tree is a valid BST
again. Do not change the shape of the tree: only the two values move. Return
the root.

Follow-up: an O(n) extra space solution is easy. Can you do it with O(h), or
even O(1), extra space?

## Example 1

```
root   = [6, 9, 2, 1, 4, 8, 12]
output = [6, 2, 9, 1, 4, 8, 12]
```

```
         6                  6
       /   \              /   \
      9     2     ->     2     9
     / \   / \          / \   / \
    1   4 8   12       1   4 8   12
```

The values `2` and `9` had traded places.

## Example 2

```
root   = [5, 4, 8, 1, 3, 7]
output = [5, 3, 8, 1, 4, 7]
```

Here the swapped nodes are a parent and child: `3` and `4`.

## Constraints

- The tree has between `2` and `1000` nodes.
- `-2^31 <= node.val <= 2^31 - 1`, and all values are distinct.
- Exactly two values are out of place.
- Trees are given (and returned) in level order; `null` marks a missing child.
