def find_duplicate_subtrees(root: "TreeNode | None") -> list["TreeNode"]:
    ids = {}     # (left id, value, right id) -> subtree id
    count = {}   # subtree id -> occurrences
    node_id = {}
    result = []
    stack = [(root, False)]
    while stack:
        node, done = stack.pop()
        if node is None:
            continue
        if not done:
            stack.append((node, True))
            stack.append((node.right, False))
            stack.append((node.left, False))
            continue
        left = node_id[id(node.left)] if node.left else 0
        right = node_id[id(node.right)] if node.right else 0
        key = (left, node.val, right)
        if key not in ids:
            ids[key] = len(ids) + 1
        sid = ids[key]
        node_id[id(node)] = sid
        count[sid] = count.get(sid, 0) + 1
        if count[sid] == 2:
            result.append(node)
    return result
