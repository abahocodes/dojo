def vertical_traversal(root: "TreeNode") -> list[list[int]]:
    entries = []  # (col, row, val)
    stack = [(root, 0, 0)]
    while stack:
        node, row, col = stack.pop()
        entries.append((col, row, node.val))
        if node.left:
            stack.append((node.left, row + 1, col - 1))
        if node.right:
            stack.append((node.right, row + 1, col + 1))
    entries.sort()
    result = []
    prev_col = None
    for col, _, val in entries:
        if col != prev_col:
            result.append([])
            prev_col = col
        result[-1].append(val)
    return result
