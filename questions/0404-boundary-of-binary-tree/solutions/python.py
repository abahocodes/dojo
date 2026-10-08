def boundary_of_binary_tree(root: "TreeNode") -> list[int]:
    def is_leaf(node):
        return node.left is None and node.right is None

    result = [root.val]
    if is_leaf(root):
        return result

    # left edge, top-down, stopping before the first leaf
    node = root.left
    while node is not None and not is_leaf(node):
        result.append(node.val)
        node = node.left if node.left is not None else node.right

    # every leaf, left to right (iterative pre-order)
    stack = [root]
    while stack:
        node = stack.pop()
        if is_leaf(node):
            result.append(node.val)
            continue
        if node.right is not None:
            stack.append(node.right)
        if node.left is not None:
            stack.append(node.left)

    # right edge, collected top-down and emitted bottom-up
    right = []
    node = root.right
    while node is not None and not is_leaf(node):
        right.append(node.val)
        node = node.right if node.right is not None else node.left
    result.extend(reversed(right))
    return result
