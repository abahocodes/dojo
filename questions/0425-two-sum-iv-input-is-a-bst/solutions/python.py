def find_target(root: "TreeNode", k: int) -> bool:
    # An inorder walk lists the values in ascending order.
    values = []
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        values.append(node.val)
        node = node.right

    i, j = 0, len(values) - 1
    while i < j:
        s = values[i] + values[j]
        if s == k:
            return True
        if s < k:
            i += 1
        else:
            j -= 1
    return False
