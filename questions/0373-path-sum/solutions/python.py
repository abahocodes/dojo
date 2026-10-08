def has_path_sum(root: "TreeNode | None", target_sum: int) -> bool:
    if root is None:
        return False
    stack = [(root, target_sum - root.val)]
    while stack:
        node, remaining = stack.pop()
        if node.left is None and node.right is None:
            if remaining == 0:
                return True
            continue
        if node.left:
            stack.append((node.left, remaining - node.left.val))
        if node.right:
            stack.append((node.right, remaining - node.right.val))
    return False
