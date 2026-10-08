def increasing_bst(root: "TreeNode") -> "TreeNode":
    dummy = TreeNode(0)
    tail = dummy
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        # its left subtree is already relinked, so the left pointer is free
        node.left = None
        tail.right = node
        tail = node
        node = node.right
    return dummy.right
