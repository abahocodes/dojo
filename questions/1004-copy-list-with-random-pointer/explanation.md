# Approach: map each original node to its copy

The only hard part is `random`: it can point forward, backward, or at the node
itself, and values repeat, so a copy's `random` target can only be found
through the **identity** of the original target. Keep a hash map from each
original node to its copy and do the work in two passes:

1. Walk the list and create `copies[node] = Node(node.val)` for every node.
2. Walk again and wire the copies through the map:
   `copies[node].next = copies[node.next]`,
   `copies[node].random = copies[node.random]`, with `None` mapping to `None`.

```python
def copy_random_list(head):
    copies = {}
    node = head
    while node is not None:
        copies[node] = Node(node.val)
        node = node.next
    node = head
    while node is not None:
        copies[node].next = copies.get(node.next)
        copies[node].random = copies.get(node.random)
        node = node.next
    return copies.get(head)
```

## Variant: O(1) extra space by interleaving

Instead of a map, splice each copy right after its original, so the list
becomes `A -> A' -> B -> B' -> ...`. Now the copy of any node `x` is simply
`x.next`, which gives the `random` wiring for free:

```python
def copy_random_list(head):
    node = head
    while node:                       # 1. interleave
        node.next = Node(node.val, node.next)
        node = node.next.next
    node = head
    while node:                       # 2. random pointers
        if node.random:
            node.next.random = node.random.next
        node = node.next.next
    dummy = tail = Node(0)
    node = head
    while node:                       # 3. unweave, restoring the original
        copy = node.next
        node.next = copy.next
        tail.next = copy
        tail = copy
        node = node.next
    return dummy.next
```

## Complexity

- Time: O(n) for both versions.
- Space: O(n) for the map; O(1) beyond the copy for the interleaving version.

## Pitfalls

- Looking up the `random` target by value breaks as soon as two nodes share a
  value (Example 2).
- Setting `copy.random = node.random` makes the copy point into the original
  list; the copy must be self-contained.
- A node whose `random` points at itself must get a copy whose `random`
  points at the copy.
- In the interleaving version, forgetting to restore the original `next`
  pointers leaves the input broken (the driver checks for this).
