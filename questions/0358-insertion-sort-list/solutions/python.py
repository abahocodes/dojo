def insertion_sort_list(head):
    dummy = ListNode(0)
    tail = None
    cur = head
    while cur:
        nxt = cur.next
        if tail is not None and tail.val <= cur.val:
            tail.next = cur
            cur.next = None
            tail = cur
        else:
            p = dummy
            while p.next and p.next.val <= cur.val:
                p = p.next
            cur.next = p.next
            p.next = cur
            if cur.next is None:
                tail = cur
        cur = nxt
    return dummy.next
