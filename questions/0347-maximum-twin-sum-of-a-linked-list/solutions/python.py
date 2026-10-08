def pair_sum(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
    prev = None
    while slow:
        nxt = slow.next
        slow.next = prev
        prev = slow
        slow = nxt
    best = 0
    a, b = head, prev
    while b:
        best = max(best, a.val + b.val)
        a, b = a.next, b.next
    return best
