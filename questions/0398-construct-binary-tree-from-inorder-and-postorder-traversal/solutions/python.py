def build_tree_in_post(inorder: list[int], postorder: list[int]) -> "TreeNode | None":
    if not postorder:
        return None
    # Walk postorder backwards (root, right, left) and inorder backwards.
    root = TreeNode(postorder[-1])
    stack = [root]
    i = len(inorder) - 1
    for value in reversed(postorder[:-1]):
        node = TreeNode(value)
        parent = stack[-1]
        if parent.val != inorder[i]:
            parent.right = node
        else:
            while stack and stack[-1].val == inorder[i]:
                parent = stack.pop()
                i -= 1
            parent.left = node
        stack.append(node)
    return root
