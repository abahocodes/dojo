def swap_pairs(head):
    dummy = ListNode(0, head)
    prev = dummy
    while prev.next is not None and prev.next.next is not None:
        a = prev.next
        b = a.next
        a.next = b.next
        b.next = a
        prev.next = b
        prev = a
    return dummy.next
