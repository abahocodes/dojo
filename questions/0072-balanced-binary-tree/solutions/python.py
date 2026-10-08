def is_balanced(root: "TreeNode | None") -> bool:
    if root is None:
        return True
    # reversed pre-order puts every child before its parent
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    height = {}
    for node in reversed(order):
        left = height.get(node.left, 0)
        right = height.get(node.right, 0)
        if abs(left - right) > 1:
            return False
        height[node] = 1 + max(left, right)
    return True
