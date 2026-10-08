def width_of_binary_tree(root: "TreeNode") -> int:
    best = 0
    level = [(root, 0)]  # (node, position within its level)
    while level:
        base = level[0][1]
        best = max(best, level[-1][1] - base + 1)
        nxt = []
        for node, pos in level:
            pos -= base  # re-index from 0 so positions stay small
            if node.left:
                nxt.append((node.left, 2 * pos))
            if node.right:
                nxt.append((node.right, 2 * pos + 1))
        level = nxt
    return best
