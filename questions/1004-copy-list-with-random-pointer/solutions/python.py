class Node:
    def __init__(self, val: int, next: "Node | None" = None, random: "Node | None" = None):
        self.val = val
        self.next = next
        self.random = random


def copy_random_list(head: Node | None) -> Node | None:
    # First pass: one new node per original node.
    copies: dict[Node, Node] = {}
    node = head
    while node is not None:
        copies[node] = Node(node.val)
        node = node.next
    # Second pass: wire next and random through the map.
    node = head
    while node is not None:
        copy = copies[node]
        copy.next = copies.get(node.next)
        copy.random = copies.get(node.random)
        node = node.next
    return copies.get(head)


# dojo calls this to run the tests; you don't need to change it.
def run_copy_random_list(nodes: list[list[int]]) -> list[list[int]]:
    originals = [Node(val) for val, _ in nodes]

    def next_of(i):
        return originals[i + 1] if i + 1 < len(originals) else None

    def random_of(r):
        return originals[r] if r >= 0 else None

    for i, (_, r) in enumerate(nodes):
        originals[i].next = next_of(i)
        originals[i].random = random_of(r)
    copy = copy_random_list(originals[0] if originals else None)

    # The original list must be left as it was.
    for i, (val, r) in enumerate(nodes):
        node = originals[i]
        if node.val != val or node.next is not next_of(i) or node.random is not random_of(r):
            return [[-1]]
    # Walk the copy: every node must be new, and the list must end.
    original_ids = {id(node) for node in originals}
    index: dict[int, int] = {}
    order = []
    node = copy
    while node is not None:
        if id(node) in original_ids or id(node) in index:
            return [[-1]]
        index[id(node)] = len(order)
        order.append(node)
        node = node.next
    result = []
    for node in order:
        if node.random is None:
            result.append([node.val, -1])
        elif id(node.random) in index:
            result.append([node.val, index[id(node.random)]])
        else:
            return [[-1]]
    return result
