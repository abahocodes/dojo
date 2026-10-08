def deserialize(data: str) -> "TreeNode | None":
    tokens = data.split(",")
    if tokens[0] == "#":
        return None
    root = TreeNode(int(tokens[0]))
    # nodes still waiting for a child, and whether their left slot is filled
    stack = [root]
    left_done = [False]
    for tok in tokens[1:]:
        child = None if tok == "#" else TreeNode(int(tok))
        parent = stack[-1]
        if not left_done[-1]:
            parent.left = child
            left_done[-1] = True
        else:
            parent.right = child
            stack.pop()
            left_done.pop()
        if child is not None:
            stack.append(child)
            left_done.append(False)
    return root
