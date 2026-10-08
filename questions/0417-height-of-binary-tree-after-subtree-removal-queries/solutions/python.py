def tree_queries(root: "TreeNode", queries: list[int]) -> list[int]:
    # preorder list of (node, depth); parents always precede children
    order = []
    stack = [(root, 0)]
    while stack:
        node, d = stack.pop()
        order.append((node, d))
        if node.left:
            stack.append((node.left, d + 1))
        if node.right:
            stack.append((node.right, d + 1))
    n = len(order)
    depth = [0] * (n + 1)
    height = [0] * (n + 1)
    for node, d in reversed(order):  # children before parents
        depth[node.val] = d
        h = 0
        if node.left:
            h = height[node.left.val] + 1
        if node.right:
            h = max(h, height[node.right.val] + 1)
        height[node.val] = h
    # per depth: the best and second-best (depth + height), and who holds the best
    levels = max(depth) + 1
    best1 = [-1] * levels
    best2 = [-1] * levels
    owner = [0] * levels
    for v in range(1, n + 1):
        d, reach = depth[v], depth[v] + height[v]
        if reach > best1[d]:
            best2[d] = best1[d]
            best1[d], owner[d] = reach, v
        elif reach > best2[d]:
            best2[d] = reach
    answer = []
    for q in queries:
        d = depth[q]
        if owner[d] != q:
            answer.append(best1[d])
        elif best2[d] >= 0:
            answer.append(best2[d])
        else:
            answer.append(d - 1)
    return answer
