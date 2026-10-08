def diameter_of_binary_tree(root: "TreeNode") -> int:
    # visit nodes so that children come before their parent (reversed pre-order)
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    height = {}  # node -> number of nodes on its longest downward path
    best = 0
    for node in reversed(order):
        left = height.get(node.left, 0)
        right = height.get(node.right, 0)
        best = max(best, left + right)
        height[node] = 1 + max(left, right)
    return best
