def path_sum(root: "TreeNode | None", target_sum: int) -> list[list[int]]:
    result = []
    if root is None:
        return result
    path = []
    total = 0
    # (node, leaving): leaving=True means "undo this node on the way back up"
    stack = [(root, False)]
    while stack:
        node, leaving = stack.pop()
        if leaving:
            path.pop()
            total -= node.val
            continue
        path.append(node.val)
        total += node.val
        stack.append((node, True))
        if node.left is None and node.right is None:
            if total == target_sum:
                result.append(list(path))
        else:
            # push right first so the left subtree is explored first
            if node.right:
                stack.append((node.right, False))
            if node.left:
                stack.append((node.left, False))
    return result
