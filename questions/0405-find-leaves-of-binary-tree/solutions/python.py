def find_leaves(root: "TreeNode") -> list[list[int]]:
    # A node is removed in round h, where h is its height (leaves have height 0).
    result = []

    def height(node):
        if node is None:
            return -1
        h = max(height(node.left), height(node.right)) + 1
        if h == len(result):
            result.append([])
        result[h].append(node.val)
        return h

    height(root)
    return result
