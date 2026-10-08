# Approach: preorder parsing with an explicit stack

In a preorder encoding, every token after the root fills the next empty child
slot of the most recent node that still has one. The left slot of a node is
always filled before its right slot, and once both slots are filled that node
is done. A stack of "open" nodes captures this exactly.

For each token:

1. Build the child (`None` for `#`).
2. Look at the top of the stack. If its left slot is still open, put the child
   there; otherwise put it in the right slot and pop the parent, since it is
   now complete.
3. If the child is a real node, push it: its children are the next tokens.

```python
def deserialize(data):
    tokens = data.split(",")
    if tokens[0] == "#":
        return None
    root = TreeNode(int(tokens[0]))
    stack, left_done = [root], [False]
    for tok in tokens[1:]:
        child = None if tok == "#" else TreeNode(int(tok))
        parent = stack[-1]
        if not left_done[-1]:
            parent.left = child
            left_done[-1] = True
        else:
            parent.right = child
            stack.pop()
            left_done.pop()
        if child is not None:
            stack.append(child)
            left_done.append(False)
    return root
```

The recursive version (`read token; if "#" return None; node.left = read();
node.right = read()`) is shorter and fine in most languages, but a
10^4-node chain can exceed Python's default recursion limit.

## Complexity

- Time: O(L), where L is the length of the string: one pass over the tokens.
- Space: O(n) for the tokens and the tree, plus O(h) for the stack.

## Pitfalls

- Negative values: parse whole tokens (split on commas), not single
  characters.
- The empty tree is `"#"`; return no node rather than crashing on `int("#")`.
- A `#` still consumes a slot. Forgetting to fill the left slot with "nothing"
  shifts every later node into the wrong place.
- Pop a node only after its right slot is filled, not after its left.
