def get_minimum_difference(root: "TreeNode") -> int:
    # Inorder lists the values in ascending order, so only neighbours matter.
    best = float("inf")
    prev = None
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        if prev is not None:
            best = min(best, node.val - prev)
        prev = node.val
        node = node.right
    return best
