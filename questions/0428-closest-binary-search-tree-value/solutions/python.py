def closest_value(root, target):
    best = root.val
    node = root
    while node:
        v = node.val
        d, bd = abs(v - target), abs(best - target)
        if d < bd or (d == bd and v < best):
            best = v
        if target < v:
            node = node.left
        elif target > v:
            node = node.right
        else:
            break
    return best
