def generate_trees(n: int) -> list["TreeNode | None"]:
    memo = {}

    def build(lo, hi):
        if lo > hi:
            return [None]
        if (lo, hi) in memo:
            return memo[(lo, hi)]
        trees = []
        for v in range(lo, hi + 1):
            for left in build(lo, v - 1):
                for right in build(v + 1, hi):
                    trees.append(TreeNode(v, left, right))
        memo[(lo, hi)] = trees
        return trees

    return build(1, n)
