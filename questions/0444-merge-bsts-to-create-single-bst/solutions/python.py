def can_merge(trees: "list[TreeNode | None]") -> "TreeNode | None":
    roots = {t.val: t for t in trees}
    leaf_values = set()
    for t in trees:
        for child in (t.left, t.right):
            if child is not None:
                leaf_values.add(child.val)

    candidates = [t for t in trees if t.val not in leaf_values]
    if len(candidates) != 1:
        return None
    root = candidates[0]
    del roots[root.val]

    # Iterative DFS carrying the open interval (lo, hi) each node must fit in.
    stack = [(root, 0, 1 << 31)]
    while stack:
        node, lo, hi = stack.pop()
        if not lo < node.val < hi:
            return None
        if node.left is None and node.right is None and node.val in roots:
            sub = roots.pop(node.val)
            node.left, node.right = sub.left, sub.right
        if node.left is not None:
            stack.append((node.left, lo, node.val))
        if node.right is not None:
            stack.append((node.right, node.val, hi))

    return root if not roots else None
