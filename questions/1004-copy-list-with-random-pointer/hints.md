# Hints

## Hint 1
Copying `val` and `next` is easy in one walk. The trouble is `random`: when
you create the copy of a node, the node its `random` points to may not have
been copied yet, and values can repeat, so you can't look the target up by
value.

## Hint 2
Separate creating nodes from wiring them. If you could answer "which copy
belongs to this original node?" in O(1), setting every `next` and `random` of
the copy would be a simple second pass.

## Hint 3
Pass one: walk the list and store `original -> new Node(original.val)` in a
hash map. Pass two: for each original node `x`, set
`copy[x].next = copy[x.next]` and `copy[x].random = copy[x.random]` (mapping
`None` to `None`). Return `copy[head]`.

## Hint 4
For O(1) extra space, make the list itself serve as the map: insert each
copy directly after its original (`A -> A' -> B -> B' ...`). Then
`x.next.random = x.random.next`, and finally unweave the two lists, restoring
the original.
