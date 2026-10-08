def is_palindrome_list(head):
    slow = fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next

    def reverse(node):
        prev = None
        while node:
            nxt = node.next
            node.next = prev
            prev = node
            node = nxt
        return prev

    tail = reverse(slow)
    ok = True
    a, b = head, tail
    while b:
        if a.val != b.val:
            ok = False
            break
        a, b = a.next, b.next
    reverse(tail)
    return ok
