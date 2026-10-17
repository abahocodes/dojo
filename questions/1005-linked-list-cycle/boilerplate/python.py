# 1005. Linked List Cycle (easy)
# /show to reread the problem · /test runs visible tests · /submit when done

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def has_cycle(head: ListNode | None) -> bool:
    pass


# dojo calls this to run the tests; you don't need to change it.
def run_has_cycle(values: list[int], pos: int) -> bool:
    nodes = [ListNode(v) for v in values]
    for a, b in zip(nodes, nodes[1:]):
        a.next = b
    if pos >= 0:
        nodes[-1].next = nodes[pos]
    return has_cycle(nodes[0] if nodes else None)
