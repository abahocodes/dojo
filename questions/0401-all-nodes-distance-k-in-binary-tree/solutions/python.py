def distance_k(root: "TreeNode | None", target: int, k: int) -> list[int]:
    # Turn the tree into an undirected graph keyed by value.
    adj = {root.val: []}
    stack = [root]
    while stack:
        node = stack.pop()
        for child in (node.left, node.right):
            if child is not None:
                adj[node.val].append(child.val)
                adj[child.val] = [node.val]
                stack.append(child)
    frontier = [target]
    seen = {target}
    for _ in range(k):
        nxt = []
        for v in frontier:
            for w in adj[v]:
                if w not in seen:
                    seen.add(w)
                    nxt.append(w)
        frontier = nxt
        if not frontier:
            break
    return frontier
