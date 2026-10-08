def good_nodes(root: "TreeNode") -> int:
    count = 0
    stack = [(root, root.val)]  # (node, largest value strictly above it on the path)
    while stack:
        node, best = stack.pop()
        if node.val >= best:
            count += 1
            best = node.val
        if node.left:
            stack.append((node.left, best))
        if node.right:
            stack.append((node.right, best))
    return count
