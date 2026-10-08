def insert_gcds(head):
    def gcd(a, b):
        while b:
            a, b = b, a % b
        return a

    cur = head
    while cur.next is not None:
        nxt = cur.next
        cur.next = ListNode(gcd(cur.val, nxt.val), nxt)
        cur = nxt
    return head
