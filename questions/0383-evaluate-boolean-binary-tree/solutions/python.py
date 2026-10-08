def evaluate_tree(root: "TreeNode") -> bool:
    # post-order without recursion: children are evaluated before parents
    value = {}
    stack = [(root, False)]
    while stack:
        node, children_done = stack.pop()
        if node.left is None:
            value[node] = node.val == 1
        elif children_done:
            left, right = value[node.left], value[node.right]
            value[node] = (left or right) if node.val == 2 else (left and right)
        else:
            stack.append((node, True))
            stack.append((node.left, False))
            stack.append((node.right, False))
    return value[root]
