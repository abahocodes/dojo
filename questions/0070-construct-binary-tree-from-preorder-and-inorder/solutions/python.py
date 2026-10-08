def build_tree(preorder: list[int], inorder: list[int]) -> "TreeNode | None":
    if not preorder:
        return None
    root = TreeNode(preorder[0])
    stack = [root]
    j = 0  # next inorder position not yet closed off
    for val in preorder[1:]:
        node = stack[-1]
        if node.val != inorder[j]:
            # node's left subtree is not finished, so val is its left child
            node.left = TreeNode(val)
            stack.append(node.left)
        else:
            # pop every node whose left side is complete; the last one popped
            # is the node whose right child val is
            while stack and stack[-1].val == inorder[j]:
                node = stack.pop()
                j += 1
            node.right = TreeNode(val)
            stack.append(node.right)
    return root
