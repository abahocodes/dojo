from collections import deque


def average_of_levels(root: "TreeNode | None") -> list[float]:
    averages = []
    queue = deque([root])
    while queue:
        size = len(queue)
        total = 0
        for _ in range(size):
            node = queue.popleft()
            total += node.val
            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)
        averages.append(total / size)
    return averages
