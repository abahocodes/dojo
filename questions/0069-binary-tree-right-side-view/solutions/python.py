def right_side_view(root: "TreeNode | None") -> list[int]:
    if root is None:
        return []
    view = []
    level = [root]
    while level:
        view.append(level[-1].val)
        nxt = []
        for node in level:
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
    return view
