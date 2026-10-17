class Node:
    def __init__(self, key: int = 0, value: int = 0):
        self.key = key
        self.value = value
        self.prev: "Node | None" = None
        self.next: "Node | None" = None


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.nodes: dict[int, Node] = {}
        # Dummy ends: head.next is the most recently used, tail.prev the least.
        self.head = Node()
        self.tail = Node()
        self.head.next = self.tail
        self.tail.prev = self.head

    def _unlink(self, node: Node) -> None:
        node.prev.next = node.next
        node.next.prev = node.prev

    def _push_front(self, node: Node) -> None:
        node.prev = self.head
        node.next = self.head.next
        self.head.next.prev = node
        self.head.next = node

    def get(self, key: int) -> int:
        node = self.nodes.get(key)
        if node is None:
            return -1
        self._unlink(node)
        self._push_front(node)
        return node.value

    def put(self, key: int, value: int) -> None:
        node = self.nodes.get(key)
        if node is not None:
            node.value = value
            self._unlink(node)
        else:
            node = Node(key, value)
            self.nodes[key] = node
            if len(self.nodes) > self.capacity:
                lru = self.tail.prev
                self._unlink(lru)
                del self.nodes[lru.key]
        self._push_front(node)


# dojo calls this to run the tests; you don't need to change it.
def lru_cache(capacity: int, ops: list[str], args: list[list[int]]) -> list[int]:
    cache = LRUCache(capacity)
    out = []
    for op, a in zip(ops, args):
        if op == "put":
            cache.put(a[0], a[1])
        else:
            out.append(cache.get(a[0]))
    return out
