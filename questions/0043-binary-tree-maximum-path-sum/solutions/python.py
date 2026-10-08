def max_path_sum(root):
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)

    gain = {}
    best = root.val
    for node in reversed(order):
        left = max(gain.get(node.left, 0), 0)
        right = max(gain.get(node.right, 0), 0)
        best = max(best, node.val + left + right)
        gain[node] = node.val + max(left, right)
    return best
