def min_cost_connect_points(points: list[list[int]]) -> int:
    n = len(points)
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    INF = float("inf")
    best = [INF] * n  # cheapest known edge from the tree to each point
    in_tree = [False] * n
    best[0] = 0
    total = 0
    for _ in range(n):
        u, cost = -1, INF
        for v in range(n):
            if not in_tree[v] and best[v] < cost:
                u, cost = v, best[v]
        in_tree[u] = True
        total += cost
        ux, uy = xs[u], ys[u]
        for v in range(n):
            if not in_tree[v]:
                d = abs(xs[v] - ux) + abs(ys[v] - uy)
                if d < best[v]:
                    best[v] = d
    return total
