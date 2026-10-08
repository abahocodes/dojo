# Approach: give every distinct subtree an integer id

Process nodes in postorder so both children already have ids. A node's
subtree is fully described by the triple `(left id, value, right id)`, using
`0` for a missing child. Map each new triple to a fresh id. Two nodes get the
same id exactly when their subtrees are identical.

Count occurrences per id; when a count reaches **2**, that node is the first
duplicate of its kind, so add it to the answer (counts of 3 or more are
ignored, so each kind appears once).

```python
def find_duplicate_subtrees(root):
    ids, count, node_id, result = {}, {}, {}, []
    stack = [(root, False)]
    while stack:
        node, done = stack.pop()
        if node is None:
            continue
        if not done:
            stack.append((node, True))
            stack.append((node.right, False))
            stack.append((node.left, False))
            continue
        left = node_id[id(node.left)] if node.left else 0
        right = node_id[id(node.right)] if node.right else 0
        key = (left, node.val, right)
        if key not in ids:
            ids[key] = len(ids) + 1
        sid = ids[key]
        node_id[id(node)] = sid
        count[sid] = count.get(sid, 0) + 1
        if count[sid] == 2:
            result.append(node)
    return result
```

**Simpler but slower:** serialize each subtree to a string such as
`"(left),val,(right)"` and count the strings. It's easy to write, but the
strings add up to O(n^2) characters on a deep tree.

## Complexity

- Time: O(n) expected: one hash lookup per node, with keys of constant size.
- Space: O(n) for the id tables and the stack.

## Pitfalls

- Serializing without null markers (or preorder alone) can make different
  shapes look equal.
- Adding a node every time its count is `>= 2` reports the same kind more than
  once.
- Recursion depth equals tree height, up to 5000 here, so an explicit stack
  is safer.
