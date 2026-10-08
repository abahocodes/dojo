def sum_of_left_leaves(root: "TreeNode | None") -> int:
    total = 0
    stack = [(root, False)]
    while stack:
        node, is_left = stack.pop()
        if node.left is None and node.right is None:
            if is_left:
                total += node.val
            continue
        if node.left:
            stack.append((node.left, True))
        if node.right:
            stack.append((node.right, False))
    return total
