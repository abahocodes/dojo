def lowest_common_ancestor(root: "TreeNode | None", p: int, q: int) -> int:
    parent = {root.val: None}
    stack = [root]
    while stack and (p not in parent or q not in parent):
        node = stack.pop()
        for child in (node.left, node.right):
            if child is not None:
                parent[child.val] = node.val
                stack.append(child)
    ancestors = set()
    while p is not None:
        ancestors.add(p)
        p = parent[p]
    while q not in ancestors:
        q = parent[q]
    return q
