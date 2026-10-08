def delete_node(root, key):
    parent, node = None, root
    while node is not None and node.val != key:
        parent = node
        node = node.left if key < node.val else node.right
    if node is None:
        return root

    if node.left is not None and node.right is not None:
        # Two children: copy the inorder successor's value, then unlink the
        # successor (it has no left child, so its right child takes its place).
        succ_parent, succ = node, node.right
        while succ.left is not None:
            succ_parent, succ = succ, succ.left
        node.val = succ.val
        if succ_parent is node:
            succ_parent.right = succ.right
        else:
            succ_parent.left = succ.right
        return root

    child = node.left if node.left is not None else node.right
    if parent is None:
        return child
    if parent.left is node:
        parent.left = child
    else:
        parent.right = child
    return root
