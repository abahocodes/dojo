def find_second_minimum_value(root: "TreeNode") -> int:
    smallest = root.val  # the root is the minimum of the whole tree
    best = -1
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val > smallest:
            # everything below is >= node.val, so no need to go deeper
            if best == -1 or node.val < best:
                best = node.val
            continue
        if node.left:
            stack.append(node.left)
            stack.append(node.right)
    return best
