# Approach: monotonic stack

Read the values in preorder. The stack holds the path of nodes that can still
receive a right child, with values decreasing from bottom to top.

- If the next value `v` is smaller than the top, it comes right after its
  parent in preorder and goes to the left: it is the top's left child.
- Otherwise `v` belongs in the right subtree of the deepest ancestor that is
  still smaller than `v`. Pop every node smaller than `v`; the last one popped
  is that ancestor, and `v` becomes its right child. Those popped nodes can
  receive no more children (their right subtrees are now closed).

Push the new node either way. Each node is pushed and popped at most once.

```python
def bst_from_preorder(preorder):
    root = TreeNode(preorder[0])
    stack = [root]
    for v in preorder[1:]:
        node = TreeNode(v)
        if v < stack[-1].val:
            stack[-1].left = node
        else:
            parent = stack.pop()
            while stack and stack[-1].val < v:
                parent = stack.pop()
            parent.right = node
        stack.append(node)
    return root
```

An equivalent O(n) recursion passes an upper bound down: `build(bound)` takes
the next value if it is below `bound`, then builds the left subtree with bound
`v` and the right subtree with the original bound. It is elegant but recurses
as deep as the tree, which can be 10^4 for a sorted input.

## Complexity

- Time: O(n): every node is pushed and popped at most once.
- Space: O(h) for the stack.

## Pitfalls

- Inserting values one by one into a BST: correct, but O(n^2) on sorted input.
- Attaching the new right child to the *first* popped node instead of the
  *last* one (the largest node still below `v`).
- Deep recursion on strictly increasing or decreasing input.
