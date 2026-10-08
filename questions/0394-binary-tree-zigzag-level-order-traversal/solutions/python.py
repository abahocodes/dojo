def zigzag_level_order(root: "TreeNode | None") -> list[list[int]]:
    result = []
    level = [root] if root else []
    left_to_right = True
    while level:
        values = [node.val for node in level]
        if not left_to_right:
            values.reverse()
        result.append(values)
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
        left_to_right = not left_to_right
    return result
