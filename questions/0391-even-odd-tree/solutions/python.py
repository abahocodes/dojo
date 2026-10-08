def is_even_odd_tree(root: "TreeNode") -> bool:
    level = [root]
    depth = 0
    while level:
        even_level = depth % 2 == 0
        prev = None
        nxt = []
        for node in level:
            v = node.val
            if even_level:
                if v % 2 == 0 or (prev is not None and v <= prev):
                    return False
            else:
                if v % 2 == 1 or (prev is not None and v >= prev):
                    return False
            prev = v
            if node.left:
                nxt.append(node.left)
            if node.right:
                nxt.append(node.right)
        level = nxt
        depth += 1
    return True
