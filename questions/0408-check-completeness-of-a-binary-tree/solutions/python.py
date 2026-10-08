from collections import deque


def is_complete_tree(root: "TreeNode") -> bool:
    # Level-order walk that also enqueues missing children. In a complete tree,
    # once the first gap appears, nothing but gaps may follow.
    queue = deque([root])
    seen_gap = False
    while queue:
        node = queue.popleft()
        if node is None:
            seen_gap = True
            continue
        if seen_gap:
            return False
        queue.append(node.left)
        queue.append(node.right)
    return True
