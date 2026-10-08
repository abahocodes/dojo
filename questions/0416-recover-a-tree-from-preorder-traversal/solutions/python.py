def recover_from_preorder(traversal: str) -> "TreeNode":
    stack = []  # stack[d] is the most recent node at depth d on the current path
    i, n = 0, len(traversal)
    while i < n:
        depth = 0
        while traversal[i] == "-":
            depth += 1
            i += 1
        value = 0
        while i < n and traversal[i] != "-":
            value = value * 10 + ord(traversal[i]) - 48
            i += 1
        node = TreeNode(value)
        del stack[depth:]
        if stack:
            parent = stack[-1]
            if parent.left is None:
                parent.left = node
            else:
                parent.right = node
        stack.append(node)
    return stack[0]
