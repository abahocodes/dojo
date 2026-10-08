def _encode(root):
    # pre-order with explicit null markers; the leading comma on every token
    # keeps "2" from matching inside "12"
    parts = []
    stack = [root]
    while stack:
        node = stack.pop()
        if node is None:
            parts.append(",#")
        else:
            parts.append("," + str(node.val))
            stack.append(node.right)
            stack.append(node.left)
    return "".join(parts)


def is_subtree(root: "TreeNode", sub_root: "TreeNode") -> bool:
    return _encode(sub_root) in _encode(root)
