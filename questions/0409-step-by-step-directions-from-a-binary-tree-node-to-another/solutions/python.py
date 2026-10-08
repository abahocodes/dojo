def get_directions(root: "TreeNode", start_value: int, dest_value: int) -> str:
    # parent[v] = (parent value, move from that parent down to v)
    parent = {root.val: (None, "")}
    stack = [root]
    while stack:
        node = stack.pop()
        if node.left is not None:
            parent[node.left.val] = (node.val, "L")
            stack.append(node.left)
        if node.right is not None:
            parent[node.right.val] = (node.val, "R")
            stack.append(node.right)

    def path_from_root(value):
        moves = []
        while parent[value][0] is not None:
            value, move = parent[value]
            moves.append(move)
        moves.reverse()
        return moves

    to_start = path_from_root(start_value)
    to_dest = path_from_root(dest_value)
    common = 0
    while (
        common < len(to_start)
        and common < len(to_dest)
        and to_start[common] == to_dest[common]
    ):
        common += 1
    return "U" * (len(to_start) - common) + "".join(to_dest[common:])
