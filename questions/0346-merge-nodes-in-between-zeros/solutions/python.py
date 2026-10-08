def merge_nodes(head):
    dummy = ListNode()
    tail = dummy
    total = 0
    cur = head.next
    while cur:
        if cur.val == 0:
            cur.val = total
            tail.next = cur
            tail = cur
            total = 0
        else:
            total += cur.val
        cur = cur.next
    tail.next = None
    return dummy.next
