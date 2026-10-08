A binary tree with distinct values has been described edge by edge. Each entry
of `descriptions` is a triple `[parent, child, is_left]` meaning:

- `child` is the **left** child of `parent` when `is_left == 1`,
- `child` is the **right** child of `parent` when `is_left == 0`.

The triples come in no particular order, and together they describe exactly
one valid binary tree (every edge of the tree appears once). Rebuild the tree
and return its root.

The returned tree is shown in level order.

## Example 1

```
descriptions = [[30, 12, 1], [12, 7, 0], [30, 45, 0], [45, 40, 1], [12, 3, 1]]
output       = [30, 12, 45, 3, 7, 40]
```

`30` never appears as a child, so it is the root. Its left child is `12` and
its right child is `45`. `12` has children `3` (left) and `7` (right), and
`40` is the left child of `45`.

## Example 2

```
descriptions = [[2, 9, 0], [5, 2, 1]]
output       = [5, 2, null, null, 9]
```

## Constraints

- `1 <= descriptions.length <= 10^4`
- `descriptions[i].length == 3`
- `1 <= parent, child <= 10^5`
- `is_left` is `0` or `1`.
- The descriptions form exactly one valid binary tree, and all node values are
  distinct.
