def partition_list(head, x):
    small = ListNode(0)
    large = ListNode(0)
    s, l = small, large
    cur = head
    while cur is not None:
        if cur.val < x:
            s.next = cur
            s = cur
        else:
            l.next = cur
            l = cur
        cur = cur.next
    l.next = None
    s.next = large.next
    return small.next
