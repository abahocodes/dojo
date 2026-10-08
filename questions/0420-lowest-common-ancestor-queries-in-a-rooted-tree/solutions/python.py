from collections import deque


def lca_queries(parent: list[int], queries: list[list[int]]) -> list[int]:
    n = len(parent)
    children = [[] for _ in range(n)]
    root = 0
    for v, p in enumerate(parent):
        if p == -1:
            root = v
        else:
            children[p].append(v)

    # BFS from the root gives every node's depth without recursion.
    depth = [0] * n
    queue = deque([root])
    while queue:
        u = queue.popleft()
        for c in children[u]:
            depth[c] = depth[u] + 1
            queue.append(c)

    # up[k][v] = the 2^k-th ancestor of v (the root points to itself).
    log = max(1, (n - 1).bit_length())
    up = [[root if p == -1 else p for p in parent]]
    for k in range(1, log):
        prev = up[k - 1]
        up.append([prev[prev[v]] for v in range(n)])

    answers = []
    for u, v in queries:
        if depth[u] < depth[v]:
            u, v = v, u
        diff = depth[u] - depth[v]
        k = 0
        while diff:
            if diff & 1:
                u = up[k][u]
            diff >>= 1
            k += 1
        if u != v:
            for k in range(log - 1, -1, -1):
                if up[k][u] != up[k][v]:
                    u = up[k][u]
                    v = up[k][v]
            u = up[0][u]
        answers.append(u)
    return answers
