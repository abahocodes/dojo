You are given the `root` of a binary search tree and an array `queries` of
positive integers. For each query `q`, report two numbers:

- the **largest** value in the tree that is `<= q`, or `-1` if no value is
  that small;
- the **smallest** value in the tree that is `>= q`, or `-1` if no value is
  that large.

Return one `[floor, ceil]` pair per query, in the same order as `queries`. When
`q` itself is in the tree, its pair is `[q, q]`.

The tree is not necessarily balanced, so walking down from the root for each
query can be slow.

## Example 1

```
root    = [6, 2, 13, 1, 4, 9, 15, null, null, null, null, null, null, 14]
queries = [2, 5, 16]
output  = [[2, 2], [4, 6], [15, -1]]
```

## Example 2

```
root    = [4, null, 9]
queries = [3]
output  = [[-1, 4]]
```

## Constraints

- The tree has between `2` and `10^5` nodes.
- `1 <= node.val <= 10^6`, and all values are distinct.
- `1 <= queries.length <= 10^5`
- `1 <= queries[i] <= 10^6`
- The input is a valid BST, given in level order with `null` for a missing
  child.
