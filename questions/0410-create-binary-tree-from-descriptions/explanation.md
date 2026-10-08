# Approach: value-to-node map plus a set of children

Because values are distinct, a value identifies a node. Keep a map from value
to node and create nodes lazily, so a value seen first as a child and later as
a parent still ends up as one node. Each triple then just sets one child
pointer.

Every node except the root has exactly one parent, so the root is the only
value that never occurs as a `child`. Collect the children in a set and pick
the parent that's not in it.

```python
def create_binary_tree(descriptions):
    nodes = {}
    children = set()

    def get(value):
        if value not in nodes:
            nodes[value] = TreeNode(value)
        return nodes[value]

    for parent, child, is_left in descriptions:
        if is_left:
            get(parent).left = get(child)
        else:
            get(parent).right = get(child)
        children.add(child)

    for parent, _, _ in descriptions:
        if parent not in children:
            return nodes[parent]
```

## Complexity

- Time: O(m), where `m` is the number of descriptions (expected O(1) per hash
  operation).
- Space: O(m) for the map and the set.

## Pitfalls

- Creating a fresh node every time a value appears. The tree then falls apart
  into disconnected pieces.
- Assuming the first triple's parent is the root: the triples are in no
  particular order.
- Mixing up the two columns: a value that never appears as a *parent* is a
  leaf. The root is the value that never appears as a *child*.
- `is_left` is an integer flag: `1` means left, `0` means right.
