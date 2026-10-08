# Approach: encode both trees, then search for a substring

A pre-order traversal that records a marker for every missing child describes a
tree completely: no other tree has the same encoding. In this encoding, any
subtree appears as one contiguous block. So `sub_root` occurs in `root` exactly
when the encoding of `sub_root` is a substring of the encoding of `root`.

Every token starts with a comma (`,5`, `,#`). Without it, `[2]` (encoded
`2,#,#`) would wrongly match inside `[12]` (`12,#,#`).

```python
def encode(root):
    parts, stack = [], [root]
    while stack:
        node = stack.pop()
        if node is None:
            parts.append(",#")
        else:
            parts.append("," + str(node.val))
            stack.append(node.right)
            stack.append(node.left)
    return "".join(parts)

def is_subtree(root, sub_root):
    return encode(sub_root) in encode(root)
```

**Simpler alternative:** for each node of `root`, check whether the tree
starting there equals `sub_root`. That is O(n * m) in the worst case, for
example when every value is the same, but it is usually fast enough in an
interview.

## Complexity

- Time: O(n + m) to build the encodings, and the substring search is linear in
  practice (KMP guarantees O(n + m)).
- Space: O(n + m) for the encodings and the traversal stacks.

## Pitfalls

- Matching only values or only shape: both must match, including the absence
  of extra children below the matched nodes (Example 2).
- Leaving out the null markers: different shapes can then share a pre-order
  sequence.
- Leaving out the separator: `2` would match inside `12`, and `-1` inside `-11`.
- Recursive comparisons on a deep tree can exceed the recursion limit.
