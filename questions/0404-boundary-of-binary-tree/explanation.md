# Approach: build the four parts separately

The boundary is the concatenation of four independent pieces, so produce them
one at a time:

1. The root's value.
2. The left edge: from `root.left`, step to the left child (or the right child
   when there is no left one) while the current node is not a leaf.
3. All leaves, left to right: an iterative pre-order traversal (push the right
   child before the left) visits them in exactly that order.
4. The right edge: the mirror loop from `root.right`, collected top-down and
   then appended in reverse.

Stopping both edge loops before a leaf is what keeps every node counted once:
the leaf at the bottom of an edge is reported by the leaf pass instead.

```python
def boundary_of_binary_tree(root):
    def is_leaf(node):
        return node.left is None and node.right is None

    result = [root.val]
    if is_leaf(root):
        return result

    node = root.left
    while node is not None and not is_leaf(node):
        result.append(node.val)
        node = node.left if node.left is not None else node.right

    stack = [root]
    while stack:
        node = stack.pop()
        if is_leaf(node):
            result.append(node.val)
            continue
        if node.right is not None:
            stack.append(node.right)
        if node.left is not None:
            stack.append(node.left)

    right = []
    node = root.right
    while node is not None and not is_leaf(node):
        right.append(node.val)
        node = node.right if node.right is not None else node.left
    result.extend(reversed(right))
    return result
```

## Complexity

- Time: O(n). The leaf pass touches every node once, and each edge loop is at
  most the height of the tree.
- Space: O(h) for the traversal stack and the right-edge buffer, plus the
  output.

## Pitfalls

- A single-node tree: the root is not a leaf here, so the answer is
  `[root.val]`, not `[root.val, root.val]`.
- The edges fall back to the *other* child when the preferred one is missing.
  Following only left children (or only right children) misses nodes like `6`
  in Example 1.
- If the root has no left child, the left edge is empty, even though the
  right subtree's nodes are on the far left of the picture. The same goes for
  the right side.
- The right edge must come out bottom-up.
- Deep, path-like trees make recursion risky in some languages; the loops and
  explicit stack above avoid that.
