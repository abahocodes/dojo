def bst_from_preorder(preorder):
    root = TreeNode(preorder[0])
    stack = [root]  # values strictly decreasing from bottom to top
    for v in preorder[1:]:
        node = TreeNode(v)
        if v < stack[-1].val:
            stack[-1].left = node
        else:
            # The parent is the last (smallest) popped node that is still < v.
            parent = stack.pop()
            while stack and stack[-1].val < v:
                parent = stack.pop()
            parent.right = node
        stack.append(node)
    return root
