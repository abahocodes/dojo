def split_list_to_parts(head, k):
    n = 0
    cur = head
    while cur:
        n += 1
        cur = cur.next
    base, extra = divmod(n, k)
    parts = []
    cur = head
    for i in range(k):
        size = base + (1 if i < extra else 0)
        part_head = cur
        for _ in range(size - 1):
            cur = cur.next
        if size > 0:
            nxt = cur.next
            cur.next = None
            cur = nxt
        parts.append(part_head)
    return parts
