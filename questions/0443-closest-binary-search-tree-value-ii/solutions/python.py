def closest_k_values(root: "TreeNode | None", target: float, k: int) -> list[int]:
    # pred holds the path to values <= target (top = largest such value),
    # succ the path to values > target (top = smallest such value).
    pred, succ = [], []
    node = root
    while node:
        if node.val <= target:
            pred.append(node)
            node = node.right
        else:
            succ.append(node)
            node = node.left

    def next_smaller():
        node = pred.pop()
        cur = node.left
        while cur:
            pred.append(cur)
            cur = cur.right
        return node.val

    def next_larger():
        node = succ.pop()
        cur = node.right
        while cur:
            succ.append(cur)
            cur = cur.left
        return node.val

    result = []
    for _ in range(k):
        if not succ or (pred and target - pred[-1].val <= succ[-1].val - target):
            result.append(next_smaller())
        else:
            result.append(next_larger())
    return result
