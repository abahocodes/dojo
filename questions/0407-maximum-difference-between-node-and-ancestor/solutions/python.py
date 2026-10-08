def max_ancestor_diff(root: "TreeNode") -> int:
    # Carry the smallest and largest value on the path from the root down.
    best = 0
    stack = [(root, root.val, root.val)]
    while stack:
        node, lo, hi = stack.pop()
        lo = min(lo, node.val)
        hi = max(hi, node.val)
        best = max(best, hi - lo)
        if node.left is not None:
            stack.append((node.left, lo, hi))
        if node.right is not None:
            stack.append((node.right, lo, hi))
    return best
