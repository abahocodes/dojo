You are given the `root` of a special binary tree of positive integers:

- every node has either **zero or two** children, and
- the value of every node with children equals the **smaller** of its two
  children's values.

Return the second smallest **distinct** value that appears anywhere in the
tree. If all nodes hold the same value, return `-1`.

## Example 1

```
root   = [3, 3, 7, null, null, 7, 9]
output = 7        # the distinct values are 3, 7 and 9
```

## Example 2

```
root   = [4, 4, 4]
output = -1       # only one distinct value
```

## Constraints

- The tree has between `1` and `25` nodes.
- `1 <= node.val <= 2^31 - 1`
- Every node has `0` or `2` children, and a parent's value equals the minimum
  of its children's values.
- The tree is given in level order; `null` marks a missing child.
