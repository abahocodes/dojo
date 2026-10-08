def preorder_traversal(root: "TreeNode | None") -> list[int]:
    result = []
    stack = [root] if root else []
    while stack:
        node = stack.pop()
        result.append(node.val)
        # Push right first so the left subtree is popped (visited) first.
        if node.right:
            stack.append(node.right)
        if node.left:
            stack.append(node.left)
    return result
