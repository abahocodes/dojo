# Approach: pre-order with an explicit token stack

The recursive definition is short:

- `f(None) = ""`
- `f(node) = val + ("(" + f(left) + ")" if left or right) + ("(" + f(right) + ")" if right)`

Concatenating strings at every level can cost O(n^2) on a deep tree, and deep
recursion can overflow the call stack. Instead, keep a stack whose entries
are either nodes still to print or literal text. When a node is popped we
emit its value and push its groups in **reverse** order (right group first),
so the left group comes out first.

```python
def tree2str(root):
    parts = []
    stack = [root]
    while stack:
        item = stack.pop()
        if isinstance(item, str):
            parts.append(item)
            continue
        parts.append(str(item.val))
        if item.right:
            stack.extend([")", item.right, "("])
        if item.left:
            stack.extend([")", item.left, "("])
        elif item.right:
            stack.append("()")
    return "".join(parts)
```

## Complexity

- Time: O(n): each node and each parenthesis is emitted once; the output has
  O(n) characters.
- Space: O(n) for the output pieces and the stack.

## Pitfalls

- Dropping the empty `()` of a missing left child when a right child exists:
  `"1(2)"` would then mean 2 is a *left* child.
- Writing `()` for a missing right child, or `()()` for a leaf.
- Negative values: print the minus sign as part of the value, `"-5()(7)"`.
- Building the string by repeated concatenation inside deep recursion.
