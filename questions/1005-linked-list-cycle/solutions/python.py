def has_cycle(head: ListNode | None) -> bool:
    # Floyd: the fast pointer laps the slow one only if there is a cycle.
    slow = fast = head
    while fast is not None and fast.next is not None:
        slow = slow.next
        fast = fast.next.next
        if slow is fast:
            return True
    return False


# dojo calls this to run the tests; you don't need to change it.
def run_has_cycle(values: list[int], pos: int) -> bool:
    nodes = [ListNode(v) for v in values]
    for a, b in zip(nodes, nodes[1:]):
        a.next = b
    if pos >= 0:
        nodes[-1].next = nodes[pos]
    return has_cycle(nodes[0] if nodes else None)
