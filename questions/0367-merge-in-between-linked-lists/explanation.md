# Approach: find the two anchors and relink

Because `1 <= a` and `b < len(list1) - 1`, the node at position `a - 1`
(`before`) and the node at position `b + 1` (`after`) always exist, and the
head of `list1` stays the head of the answer.

1. Walk `a - 1` steps from the head to reach `before`.
2. Continue `b - a + 2` steps from `before` to reach `after`.
3. Link `before.next = list2`, walk to the tail of `list2` and link
   `tail.next = after`.

```python
def merge_in_between(list1, a, b, list2):
    before = list1
    for _ in range(a - 1):
        before = before.next
    after = before
    for _ in range(b - a + 2):
        after = after.next
    before.next = list2
    tail = list2
    while tail.next:
        tail = tail.next
    tail.next = after
    return list1
```

## Complexity

- Time: O(n + m) for the two walks.
- Space: O(1).

## Pitfalls

- Off-by-one errors: you need the node *before* position `a` and the node
  *after* position `b`, not the nodes at `a` and `b`.
- Linking `before.next = list2` before you have found `after` loses access to
  the rest of `list1` — find both anchors first.
- Forgetting to walk `list2` to its tail and attaching `after` to its head.
