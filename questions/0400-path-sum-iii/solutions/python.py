def path_sum_count(root: "TreeNode | None", target_sum: int) -> int:
    if root is None:
        return 0
    seen = {0: 1}  # prefix sums on the current root-to-node path
    count = 0
    stack = [(root, 0, False)]
    while stack:
        node, before, leaving = stack.pop()
        prefix = before + node.val
        if leaving:
            seen[prefix] -= 1
            continue
        count += seen.get(prefix - target_sum, 0)
        seen[prefix] = seen.get(prefix, 0) + 1
        stack.append((node, before, True))
        if node.right is not None:
            stack.append((node.right, prefix, False))
        if node.left is not None:
            stack.append((node.left, prefix, False))
    return count
