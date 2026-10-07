def max_depth(root: "TreeNode | None") -> int:
    if root is None:
        return 0
    depth = 0
    level = [root]
    while level:
        depth += 1
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
    return depth
