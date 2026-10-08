def inorder_successor(root, p):
    answer = -1
    node = root
    while node:
        if node.val > p:
            answer = node.val  # candidate; look for a smaller one on the left
            node = node.left
        else:
            node = node.right
    return answer
