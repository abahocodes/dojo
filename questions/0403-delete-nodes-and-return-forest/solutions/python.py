def del_nodes(root: "TreeNode | None", to_delete: list[int]) -> list["TreeNode"]:
    doomed = set(to_delete)
    forest = []
    # Each entry: (node, is it the top of a tree once its parent is gone?)
    stack = [(root, True)]
    while stack:
        node, is_root = stack.pop()
        deleted = node.val in doomed
        if is_root and not deleted:
            forest.append(node)
        for child in (node.left, node.right):
            if child is not None:
                stack.append((child, deleted))
        if node.left is not None and node.left.val in doomed:
            node.left = None
        if node.right is not None and node.right.val in doomed:
            node.right = None
    return forest
