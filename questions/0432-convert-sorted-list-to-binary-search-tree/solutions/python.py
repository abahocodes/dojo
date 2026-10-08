def sorted_list_to_bst(head):
    n = 0
    node = head
    while node:
        n += 1
        node = node.next

    cur = head

    def build(lo, hi):
        # Builds positions lo..hi in inorder, consuming list nodes as it goes.
        nonlocal cur
        if lo > hi:
            return None
        mid = (lo + hi + 1) // 2
        left = build(lo, mid - 1)
        root = TreeNode(cur.val, left)
        cur = cur.next
        root.right = build(mid + 1, hi)
        return root

    return build(0, n - 1)
