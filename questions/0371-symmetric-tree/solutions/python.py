def is_symmetric(root):
    stack = [(root.left, root.right)]
    while stack:
        a, b = stack.pop()
        if a is None and b is None:
            continue
        if a is None or b is None or a.val != b.val:
            return False
        stack.append((a.left, b.right))
        stack.append((a.right, b.left))
    return True
