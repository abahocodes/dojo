def find_mode(root: "TreeNode") -> list[int]:
    # Inorder visits equal values consecutively, so a running count suffices.
    modes = []
    best = 0
    count = 0
    prev = None
    stack, node = [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        count = count + 1 if node.val == prev else 1
        prev = node.val
        if count > best:
            best = count
            modes = [node.val]
        elif count == best:
            modes.append(node.val)
        node = node.right
    return modes
