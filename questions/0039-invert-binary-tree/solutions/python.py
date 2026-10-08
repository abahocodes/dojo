def invert_tree(root):
    stack = [root]
    while stack:
        node = stack.pop()
        if node:
            node.left, node.right = node.right, node.left
            stack.append(node.left)
            stack.append(node.right)
    return root
