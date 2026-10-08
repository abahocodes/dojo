def average_of_subtree(root: "TreeNode") -> int:
    # Preorder puts every parent before its children, so walking it
    # backwards visits children first (a post-order substitute).
    order = []
    stack = [root]
    while stack:
        node = stack.pop()
        order.append(node)
        if node.left:
            stack.append(node.left)
        if node.right:
            stack.append(node.right)
    totals = {}  # node -> (sum, count) of its subtree
    count = 0
    for node in reversed(order):
        s, c = node.val, 1
        for child in (node.left, node.right):
            if child:
                cs, cc = totals[child]
                s += cs
                c += cc
        totals[node] = (s, c)
        if s // c == node.val:
            count += 1
    return count
