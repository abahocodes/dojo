You are given the `root` of a binary tree. Rearrange its nodes, in place, into a
single right-leaning chain that lists the nodes in **preorder** (node, then its
left subtree, then its right subtree):

- every node's `left` pointer becomes `null`;
- every node's `right` pointer points to the node that follows it in preorder,
  and the last node's `right` is `null`.

Return the root of the rearranged tree (which is still the original root). An
empty tree stays empty.

## Example 1

```
root   = [1, 2, 5, 3, 4, null, 6]
output = [1, null, 2, null, 3, null, 4, null, 5, null, 6]
```

## Example 2

```
root   = []
output = []
```

## Constraints

- The tree has between `0` and `2000` nodes.
- `-100 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.

**Follow-up:** can you do it with O(1) extra space?
