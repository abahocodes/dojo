def convert_bst(root: "TreeNode | None") -> "TreeNode | None":
    running = 0
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.right
        node = stack.pop()
        running += node.val
        node.val = running
        node = node.left
    return root
