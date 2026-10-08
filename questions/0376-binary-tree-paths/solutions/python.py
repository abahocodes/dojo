def binary_tree_paths(root: "TreeNode | None") -> list[str]:
    paths = []
    stack = [(root, str(root.val))]
    while stack:
        node, path = stack.pop()
        if node.left is None and node.right is None:
            paths.append(path)
            continue
        if node.right:
            stack.append((node.right, path + "->" + str(node.right.val)))
        if node.left:
            stack.append((node.left, path + "->" + str(node.left.val)))
    return paths
