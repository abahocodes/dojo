def create_binary_tree(descriptions: list[list[int]]) -> "TreeNode":
    nodes = {}
    children = set()

    def get(value):
        if value not in nodes:
            nodes[value] = TreeNode(value)
        return nodes[value]

    for parent, child, is_left in descriptions:
        if is_left:
            get(parent).left = get(child)
        else:
            get(parent).right = get(child)
        children.add(child)

    # the root is the only value that never appears as a child
    for parent, _, _ in descriptions:
        if parent not in children:
            return nodes[parent]
    return None
