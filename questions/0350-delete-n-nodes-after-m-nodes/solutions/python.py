def delete_nodes(head, m, n):
    cur = head
    while cur is not None:
        for _ in range(m - 1):
            if cur.next is None:
                return head
            cur = cur.next
        skip = cur.next
        for _ in range(n):
            if skip is None:
                break
            skip = skip.next
        cur.next = skip
        cur = skip
    return head
