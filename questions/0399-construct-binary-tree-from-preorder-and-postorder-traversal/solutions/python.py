def construct_from_pre_post(preorder: list[int], postorder: list[int]) -> "TreeNode | None":
    root = TreeNode(preorder[0])
    stack = [root]
    j = 0
    for value in preorder[1:]:
        node = TreeNode(value)
        # Pop every node whose subtree is already complete.
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
