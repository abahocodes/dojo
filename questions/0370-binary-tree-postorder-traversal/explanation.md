# Approach: reversed "node, right, left" preorder

Postorder is *left, right, node*. Read backwards, that is *node, right, left*:
a preorder traversal that visits the right child before the left. Preorder is
simple with a stack (pop, record, push the children), so collect the values in
*node, right, left* order and reverse the list at the end.

```python
def postorder_traversal(root):
    if root is None:
        return []
    result = []
    stack = [root]
    while stack:
        node = stack.pop()
        result.append(node.val)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    result.reverse()
    return result
```

The left child is pushed first so that the right child is on top and is
processed first.

An alternative that emits values in order (useful when you need to *act* on
nodes in true postorder) walks left like the inorder traversal and keeps a
`last_visited` pointer: a node on top of the stack is emitted only when it has
no right child or its right child was the node visited last.

## Complexity

- Time: O(n).
- Space: O(n) for the stack and the result list.

## Pitfalls

- Pushing the right child first and then the left gives plain preorder, which
  reversed is not postorder.
- Forgetting the final reversal.
- A recursive solution can overflow the call stack on a `10^4`-level-deep
  tree.
