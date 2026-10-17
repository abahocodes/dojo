You are given `head`, the first node of a singly linked list (or `None` for an
empty list). Each node has a value `val` and a pointer `next`. Normally the
last node's `next` is `None`, but this list might be broken: the last node
may instead point back at some earlier node (or at itself), so following
`next` would go around forever.

Implement `has_cycle(head)`: return `True` if following `next` pointers from
`head` ever revisits a node, and `False` if it reaches the end of the list.

Values may repeat, so two nodes with the same `val` are not necessarily the
same node.

## How the tests describe a list

Each test gives `values`, the node values in `next` order, and `pos`, the
0-based index of the node the last node's `next` points back to, or `-1` if
the list ends normally. The provided driver, `run_has_cycle`, builds that list
and returns what your `has_cycle` returns. Your function only sees `head`; it
is not told `pos`. You don't need to change the driver.

## Example 1

```
values = [4, 8, 15, 16, 23]
pos    = 1
output = true
```

The list is `4 -> 8 -> 15 -> 16 -> 23`, and `23` points back at `8`.

## Example 2

```
values = [4, 8, 15, 16, 23]
pos    = -1
output = false
```

## Example 3

```
values = [9]
pos    = 0
output = true
```

A single node whose `next` points at itself.

## Constraints

- The list has between `0` and `10^4` nodes.
- `-10^5 <= node.val <= 10^5`
- `pos` is `-1` or a valid index into `values`.

**Follow-up:** can you solve it using O(1) extra memory?
