def double_it(head):
    if head.val >= 5:
        head = ListNode(0, head)
    node = head
    while node:
        node.val = (node.val * 2) % 10
        if node.next and node.next.val >= 5:
            node.val += 1
        node = node.next
    return head
