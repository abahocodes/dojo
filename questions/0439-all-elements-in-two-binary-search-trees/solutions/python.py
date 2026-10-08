def get_all_elements(root1: "TreeNode | None", root2: "TreeNode | None") -> list[int]:
    def inorder(root):
        values, stack, node = [], [], root
        while stack or node:
            while node:
                stack.append(node)
                node = node.left
            node = stack.pop()
            values.append(node.val)
            node = node.right
        return values

    a, b = inorder(root1), inorder(root2)
    merged = []
    i = j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            merged.append(a[i])
            i += 1
        else:
            merged.append(b[j])
            j += 1
    merged.extend(a[i:])
    merged.extend(b[j:])
    return merged
