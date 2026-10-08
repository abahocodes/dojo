from bisect import bisect_left


def closest_nodes(root: "TreeNode | None", queries: list[int]) -> list[list[int]]:
    values, stack, node = [], [], root
    while stack or node:
        while node:
            stack.append(node)
            node = node.left
        node = stack.pop()
        values.append(node.val)
        node = node.right

    answer = []
    for q in queries:
        i = bisect_left(values, q)
        if i < len(values) and values[i] == q:
            answer.append([q, q])
            continue
        floor = values[i - 1] if i > 0 else -1
        ceil = values[i] if i < len(values) else -1
        answer.append([floor, ceil])
    return answer
