def amount_of_time(root: "TreeNode", start: int) -> int:
    # Record parents so the tree can be walked as an undirected graph.
    parent = {root: None}
    source = None
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val == start:
            source = node
        for child in (node.left, node.right):
            if child is not None:
                parent[child] = node
                stack.append(child)

    # BFS outwards from the start node, one minute per layer.
    seen = {source}
    frontier = [source]
    minutes = -1
    while frontier:
        minutes += 1
        nxt = []
        for node in frontier:
            for neighbor in (node.left, node.right, parent[node]):
                if neighbor is not None and neighbor not in seen:
                    seen.add(neighbor)
                    nxt.append(neighbor)
        frontier = nxt
    return minutes
