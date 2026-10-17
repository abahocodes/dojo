class Node:
    def __init__(self, val: int = 0, neighbors: "list[Node] | None" = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


def clone_graph(node: Node | None) -> Node | None:
    if node is None:
        return None
    # Iterative DFS; a node gets its copy the first time it is seen.
    copies = {node: Node(node.val)}
    stack = [node]
    while stack:
        cur = stack.pop()
        for nb in cur.neighbors:
            if nb not in copies:
                copies[nb] = Node(nb.val)
                stack.append(nb)
            copies[cur].neighbors.append(copies[nb])
    return copies[node]


# dojo calls this to run the tests; you don't need to change it.
def run_clone_graph(adj_list: list[list[int]]) -> list[list[int]]:
    n = len(adj_list)
    originals = [Node(v + 1) for v in range(n)]
    for node, nbrs in zip(originals, adj_list):
        node.neighbors = [originals[v - 1] for v in nbrs]
    clone = clone_graph(originals[0] if originals else None)

    # The original graph must be left as it was.
    for i, (node, nbrs) in enumerate(zip(originals, adj_list)):
        if (node.val != i + 1 or len(node.neighbors) != len(nbrs)
                or any(a is not originals[v - 1] for a, v in zip(node.neighbors, nbrs))):
            return [[-1]]
    if clone is None:
        return []
    # Explore the clone: no original node, one node per value 1..n.
    original_ids = {id(node) for node in originals}
    seen: set[int] = set()
    by_val: dict[int, Node] = {}
    stack = [clone]
    while stack:
        node = stack.pop()
        if id(node) in original_ids:
            return [[-1]]
        if id(node) in seen:
            continue
        seen.add(id(node))
        if node.val in by_val or not 1 <= node.val <= n:
            return [[-1]]
        by_val[node.val] = node
        stack.extend(node.neighbors)
    return [sorted(nb.val for nb in by_val[v].neighbors) if v in by_val else [] for v in range(1, n + 1)]
