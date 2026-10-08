def deepest_leaves_sum(root: "TreeNode") -> int:
    level = [root]
    while True:
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        if not nxt:
            return sum(node.val for node in level)
        level = nxt
