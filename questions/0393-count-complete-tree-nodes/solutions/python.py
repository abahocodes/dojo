def count_nodes(root: "TreeNode | None") -> int:
    def left_depth(node):
        depth = 0
        while node:
            depth += 1
            node = node.left
        return depth

    count = 0
    node = root
    while node:
        lh = left_depth(node.left)
        rh = left_depth(node.right)
        if lh == rh:
            # Left subtree is perfect with height lh: 2^lh - 1 nodes, plus node.
            count += 1 << lh
            node = node.right
        else:
            # Right subtree is perfect with height rh.
            count += 1 << rh
            node = node.left
    return count
