def is_cousins(root: "TreeNode | None", x: int, y: int) -> bool:
    level = [root]
    while level:
        parent_x = parent_y = None
        next_level = []
        for node in level:
            for child in (node.left, node.right):
                if child is None:
                    continue
                if child.val == x:
                    parent_x = node
                elif child.val == y:
                    parent_y = node
                next_level.append(child)
        if parent_x and parent_y:
            return parent_x is not parent_y
        if parent_x or parent_y:
            return False
        level = next_level
    return False
