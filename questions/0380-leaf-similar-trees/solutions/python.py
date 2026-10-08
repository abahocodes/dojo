def leaf_similar(root1: "TreeNode", root2: "TreeNode") -> bool:
    def leaves(root):
        out = []
        stack = [root] if root else []
        while stack:
            node = stack.pop()
            if node.left is None and node.right is None:
                out.append(node.val)
                continue
            # push right first so the left subtree is visited first
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)
        return out

    return leaves(root1) == leaves(root2)
