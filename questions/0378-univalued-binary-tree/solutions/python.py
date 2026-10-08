def is_unival_tree(root: "TreeNode | None") -> bool:
    value = root.val
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val != value:
            return False
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    return True
