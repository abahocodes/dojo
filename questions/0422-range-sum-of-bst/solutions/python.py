def range_sum_bst(root: "TreeNode | None", low: int, high: int) -> int:
    total = 0
    stack = [root] if root else []
    while stack:
        node = stack.pop()
        if node.val < low:
            # everything on the left is even smaller
            if node.right:
                stack.append(node.right)
        elif node.val > high:
            # everything on the right is even larger
            if node.left:
                stack.append(node.left)
        else:
            total += node.val
            if node.left:
                stack.append(node.left)
            if node.right:
                stack.append(node.right)
    return total
