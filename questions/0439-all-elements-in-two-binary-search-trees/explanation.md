# Approach: two in-order traversals, then a merge

Each BST's in-order traversal is already sorted, so the task reduces to
merging two sorted lists, the merge step of merge sort. Iterative traversals
keep deep (chain-shaped) trees safe.

```python
def get_all_elements(root1, root2):
    def inorder(root):
        values, stack, node = [], [], root
        while stack or node:
            while node:
                stack.append(node)
                node = node.left
            node = stack.pop()
            values.append(node.val)
            node = node.right
        return values

    a, b = inorder(root1), inorder(root2)
    merged = []
    i = j = 0
    while i < len(a) and j < len(b):
        if a[i] <= b[j]:
            merged.append(a[i])
            i += 1
        else:
            merged.append(b[j])
            j += 1
    merged.extend(a[i:])
    merged.extend(b[j:])
    return merged
```

A variant interleaves the two traversals directly with two stacks, avoiding
the intermediate lists; the complexity is the same.

## Complexity

- Time: O(n + m) for trees with n and m nodes.
- Space: O(n + m) for the output (plus the two traversal lists).

## Pitfalls

- Concatenating and calling a sort works but is O((n + m) log(n + m)) and
  ignores the BST structure.
- Do not drop duplicates: a value in both trees must appear twice.
- Remember to append the leftover tail of whichever list is not exhausted,
  and handle empty trees.
