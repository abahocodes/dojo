def lowest_common_ancestor(root, p, q):
    lo, hi = min(p, q), max(p, q)
    node = root
    while True:
        if hi < node.val:
            node = node.left
        elif lo > node.val:
            node = node.right
        else:
            return node.val
