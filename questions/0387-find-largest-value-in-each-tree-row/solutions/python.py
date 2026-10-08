def largest_values(root: "TreeNode") -> list[int]:
    result = []
    level = [root] if root else []
    while level:
        result.append(max(node.val for node in level))
        next_level = []
        for node in level:
            if node.left:
                next_level.append(node.left)
            if node.right:
                next_level.append(node.right)
        level = next_level
    return result
