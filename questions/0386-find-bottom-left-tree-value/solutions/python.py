from collections import deque


def find_bottom_left_value(root: "TreeNode") -> int:
    # BFS that enqueues the right child before the left one:
    # the last node dequeued is the leftmost node of the deepest level
    queue = deque([root])
    node = root
    while queue:
        node = queue.popleft()
        if node.right:
            queue.append(node.right)
        if node.left:
            queue.append(node.left)
    return node.val
