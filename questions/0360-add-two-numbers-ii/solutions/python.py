def add_two_numbers_ii(l1, l2):
    a, b = [], []
    while l1:
        a.append(l1.val)
        l1 = l1.next
    while l2:
        b.append(l2.val)
        l2 = l2.next
    head = None
    carry = 0
    while a or b or carry:
        s = carry + (a.pop() if a else 0) + (b.pop() if b else 0)
        head = ListNode(s % 10, head)
        carry = s // 10
    return head
