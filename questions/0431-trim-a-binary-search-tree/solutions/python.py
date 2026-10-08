def trim_bst(root, low, high):
    # Find the new root: the first node on the search path inside [low, high].
    while root is not None and (root.val < low or root.val > high):
        root = root.right if root.val < low else root.left
    if root is None:
        return None

    # Left spine: everything here is < root.val <= high, so only `low` matters.
    node = root
    while node.left is not None:
        if node.left.val < low:
            node.left = node.left.right
        else:
            node = node.left

    # Right spine: everything here is > root.val >= low, so only `high` matters.
    node = root
    while node.right is not None:
        if node.right.val > high:
            node.right = node.right.left
        else:
            node = node.right
    return root
