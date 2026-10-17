# Approach: hash map plus doubly linked list

Two structures, each covering what the other can't:

- a **doubly linked list** of entries ordered by recency: the most recently
  used next to a dummy `head`, the least recently used next to a dummy `tail`;
- a **hash map** from key to its list node.

Every operation is a few pointer updates:

- `get(key)`: if the key is missing return `-1`; otherwise unlink its node,
  re-insert it right after `head`, and return its value.
- `put(key, value)`: if the key exists, update the node's value and move it
  after `head`. Otherwise create a node after `head` and add it to the map; if
  the map now holds more than `capacity` keys, the node just before `tail` is
  the least recently used: unlink it and delete its key from the map.

```python
class Node:
    def __init__(self, key=0, value=0):
        self.key, self.value = key, value
        self.prev = self.next = None

class LRUCache:
    def __init__(self, capacity):
        self.capacity = capacity
        self.nodes = {}
        self.head, self.tail = Node(), Node()   # dummies
        self.head.next, self.tail.prev = self.tail, self.head

    def _unlink(self, node):
        node.prev.next, node.next.prev = node.next, node.prev

    def _push_front(self, node):
        node.prev, node.next = self.head, self.head.next
        self.head.next.prev = node
        self.head.next = node

    def get(self, key):
        node = self.nodes.get(key)
        if node is None:
            return -1
        self._unlink(node)
        self._push_front(node)
        return node.value

    def put(self, key, value):
        node = self.nodes.get(key)
        if node is not None:
            node.value = value
            self._unlink(node)
        else:
            node = self.nodes[key] = Node(key, value)
            if len(self.nodes) > self.capacity:
                lru = self.tail.prev
                self._unlink(lru)
                del self.nodes[lru.key]
        self._push_front(node)
```

Each node stores its key so the eviction step can delete it from the map.

Many languages ship this combination ready-made: Python's `OrderedDict`
(`move_to_end`, `popitem(last=False)`), Java's `LinkedHashMap` in access
order, and JavaScript's `Map`, which iterates in insertion order (delete and
re-set a key to mark it as used; the first key is the oldest). Interviewers
often accept these but usually want to see the hand-built version.

## Complexity

- Time: O(1) average for `get` and `put`.
- Space: O(capacity).

## Pitfalls

- A `get` is a use: forgetting to move the node on a hit evicts the wrong key.
- Updating an existing key must not count as a new entry or trigger an
  eviction.
- Without dummy head/tail nodes, inserting into an empty list and removing
  the last node need special cases that are easy to get wrong.
- Store the key in the node; otherwise eviction can't find the map entry to
  delete.
- A plain list or array ordered by recency makes each operation O(capacity).
