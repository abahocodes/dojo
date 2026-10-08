def reverse_between(head, left, right):
    dummy = ListNode(0, head)
    before = dummy
    for _ in range(left - 1):
        before = before.next
    tail = before.next
    for _ in range(right - left):
        move = tail.next
        tail.next = move.next
        move.next = before.next
        before.next = move
    return dummy.next
