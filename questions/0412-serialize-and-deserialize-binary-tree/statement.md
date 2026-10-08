A binary tree was flattened into a string by a preorder walk: each node is
written as its integer value, and every missing child is written as `#`.
Tokens are separated by single commas, with no spaces. For example, a lone
node with value `6` becomes `"6,#,#"`, and the empty tree becomes `"#"`.

Given such a string `data`, rebuild the tree it describes and return its root
(return an empty tree for `"#"`).

## Example 1

```
data   = "1,2,#,#,3,4,#,#,5,#,#"
output = [1, 2, 3, null, null, 4, 5]
```

Node `1` has left child `2` (a leaf) and right child `3`, whose children are
the leaves `4` and `5`.

## Example 2

```
data   = "7,-3,#,12,0,#,#,#,#"
output = [7, -3, null, null, 12, 0]
```

`-3` has no left child; its right child `12` has a left child `0`. Node `7`
has no right child.

## Constraints

- `data` is a valid preorder encoding of some binary tree.
- The tree has between `0` and `10^4` nodes.
- `-1000 <= node.val <= 1000`
- The returned tree is shown in level order with `null` for missing children.
