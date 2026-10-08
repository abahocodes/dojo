# Approach: preorder with a stack, closed off by postorder

Preorder gives the order in which nodes are created. Postorder tells you when a
node is finished: a node appears in `postorder` only after its whole subtree.
So walk `preorder` and keep the current root-to-node path on a stack:

- Before placing a new value, pop every stack node that equals `postorder[j]`
  (advancing `j`). Those subtrees are complete and take no more children.
- The stack top is now the new node's parent. Fill its left slot first, then
  its right slot. That is exactly the "lone child goes left" rule.

```python
def construct_from_pre_post(preorder, postorder):
    root = TreeNode(preorder[0])
    stack = [root]
    j = 0
    for value in preorder[1:]:
        node = TreeNode(value)
        while stack[-1].val == postorder[j]:
            stack.pop()
            j += 1
        parent = stack[-1]
        if parent.left is None:
            parent.left = node
        else:
            parent.right = node
        stack.append(node)
    return root
```

**Alternative (recursive):** the root is `pre[0]`; if there are more values,
`pre[1]` is the left child. Its index `k` in `post` gives the left subtree size
`k + 1`. Recurse on `pre[1:k+2]` / `post[:k+1]` for the left subtree and
`pre[k+2:]` / `post[k+1:-1]` for the right one. A value-to-index map over
`postorder` makes this O(n).

## Complexity

- Time: O(n): each node is pushed and popped once.
- Space: O(n) for the stack and the tree.

## Pitfalls

- With one child there's no way to tell left from right, so the tie-break
  rule matters. Putting a lone child on the right builds a tree that matches
  both traversals but isn't the expected answer.
- The stack top never runs out: the root is the last value in `postorder`, so
  it is never popped while preorder values remain.
