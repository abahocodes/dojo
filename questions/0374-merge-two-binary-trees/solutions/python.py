def merge_trees(root1: "TreeNode | None", root2: "TreeNode | None") -> "TreeNode | None":
    if root1 is None:
        return root2
    stack = [(root1, root2)]
    while stack:
        a, b = stack.pop()
        if b is None:
            continue
        a.val += b.val
        if a.left is None:
            a.left = b.left
        else:
            stack.append((a.left, b.left))
        if a.right is None:
            a.right = b.right
        else:
            stack.append((a.right, b.right))
    return root1
