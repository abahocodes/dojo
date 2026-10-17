Each node of a singly linked list holds a value `val`, the usual `next`
pointer, and one extra pointer, `random`, which may point at **any** node of
the same list (itself included) or be `None`.

Implement `copy_random_list(head)`: build a **deep copy** of the list and
return the head of the copy. The copy must consist entirely of brand-new nodes,
one per original node, with the same values, and its `next` and `random`
pointers must point at nodes of the copy in the same pattern as the original.
No pointer of the copy may lead back into the original list, and the original
list must be left unchanged when you return. An empty list (`head` is `None`)
copies to `None`.

The `Node` class is defined at the top of the starter code.

## How the tests describe a list

A list is written as `nodes`, one `[val, random_index]` pair per node in
`next` order, where `random_index` is the position (0-based) of the node that
`random` points to, or `-1` for `None`. The provided driver,
`run_copy_random_list`, builds that list, calls your `copy_random_list`, and
writes your copy back in the same format, so a correct copy reproduces the
input exactly. If the copy reuses any original node, points into the original
list, or the original list was changed, the driver returns `[[-1]]` instead.
You don't need to change the driver.

## Example 1

```
nodes  = [[7, -1], [13, 0], [11, 4], [10, 2], [1, 0]]
output = [[7, -1], [13, 0], [11, 4], [10, 2], [1, 0]]
```

The list is `7 -> 13 -> 11 -> 10 -> 1`. The `13` and the `1` both have
`random` pointing at the head, `11` points at the tail, `10` at `11`, and the
head's `random` is `None`. The copy must have the same shape.

## Example 2

```
nodes  = [[3, 2], [3, 0], [3, -1]]
output = [[3, 2], [3, 0], [3, -1]]
```

All values are equal, so nodes have to be matched by identity, not by value.

## Example 3

```
nodes  = [[5, 0]]
output = [[5, 0]]
```

A single node whose `random` points at itself; the copy's `random` must point
at the copy, not at the original.

## Constraints

- The list has between `0` and `1000` nodes.
- `-10^4 <= node.val <= 10^4`
- Each `random` is `None` or points at a node of the same list.

**Follow-up:** can you do it with O(1) extra memory besides the copy itself?
