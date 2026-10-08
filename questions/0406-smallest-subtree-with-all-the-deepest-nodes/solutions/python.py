def subtree_with_all_deepest(root: "TreeNode") -> "TreeNode":
    # dfs(node) -> (height of the subtree, root of the answer inside it)
    def dfs(node):
        if node is None:
            return 0, None
        left_h, left_ans = dfs(node.left)
        right_h, right_ans = dfs(node.right)
        if left_h > right_h:
            return left_h + 1, left_ans
        if right_h > left_h:
            return right_h + 1, right_ans
        return left_h + 1, node

    return dfs(root)[1]
