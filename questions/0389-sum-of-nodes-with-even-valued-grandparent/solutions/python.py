def sum_even_grandparent(root: "TreeNode") -> int:
    total = 0
    stack = [root]
    while stack:
        node = stack.pop()
        for child in (node.left, node.right):
            if child is None:
                continue
            if node.val % 2 == 0:
                if child.left:
                    total += child.left.val
                if child.right:
                    total += child.right.val
            stack.append(child)
    return total
