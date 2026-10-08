def flatten(root: "TreeNode | None") -> "TreeNode | None":
    node = root
    while node is not None:
        if node.left is not None:
            # Splice the left subtree between node and its right subtree.
            tail = node.left
            while tail.right is not None:
                tail = tail.right
            tail.right = node.right
            node.right = node.left
            node.left = None
        node = node.right
    return root
