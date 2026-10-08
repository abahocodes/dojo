def is_valid_bst(root):
    stack = []
    prev = None
    node = root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        if prev is not None and node.val <= prev:
            return False
        prev = node.val
        node = node.right
    return True
