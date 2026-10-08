def rotate_right(head, k):
    if head is None:
        return None
    n = 1
    tail = head
    while tail.next is not None:
        tail = tail.next
        n += 1
    r = k % n
    if r == 0:
        return head
    tail.next = head
    new_tail = head
    for _ in range(n - r - 1):
        new_tail = new_tail.next
    new_head = new_tail.next
    new_tail.next = None
    return new_head
