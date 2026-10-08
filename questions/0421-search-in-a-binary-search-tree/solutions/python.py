def search_bst(root: "TreeNode | None", val: int) -> "TreeNode | None":
    node = root
    while node is not None and node.val != val:
        node = node.left if val < node.val else node.right
    return node
