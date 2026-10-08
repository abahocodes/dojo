def find_tilt(root: "TreeNode") -> int:
    if root is None:
        return 0
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
    subtree_sum = {}
    tilt = 0
    for node in reversed(order):
        left = subtree_sum.get(node.left, 0)
        right = subtree_sum.get(node.right, 0)
        tilt += abs(left - right)
        subtree_sum[node] = node.val + left + right
    return tilt
