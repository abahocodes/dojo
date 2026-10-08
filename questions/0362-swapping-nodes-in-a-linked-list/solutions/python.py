def swap_nodes(head, k):
    first = head
    for _ in range(k - 1):
        first = first.next
    runner, second = first, head
    while runner.next:
        runner = runner.next
        second = second.next
    first.val, second.val = second.val, first.val
    return head
