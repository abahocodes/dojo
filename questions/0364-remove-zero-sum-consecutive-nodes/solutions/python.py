def remove_zero_sum_sublists(head):
    dummy = ListNode(0, head)
    last = {}
    total = 0
    node = dummy
    while node:
        total += node.val
        last[total] = node
        node = node.next
    total = 0
    node = dummy
    while node:
        total += node.val
        node.next = last[total].next
        node = node.next
    return dummy.next
