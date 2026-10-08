You are given the `root` of a non-empty binary tree. For every **leaf** (a node
with no children), write down the path from the root to that leaf as a string:
the node values in order from the root down, joined by `"->"`, with no spaces.
For example, the path through the values `1`, `2` and `5` is `"1->2->5"`, and a
negative value is written with its minus sign, as in `"4->-3"`.

Return one string per leaf. The strings may be in any order.

## Example 1

```
root   = [1, 2, 3, null, 5]
output = ["1->2->5", "1->3"]
```

## Example 2

```
root   = [4, -3]
output = ["4->-3"]      # 4 has one child, so the only leaf is -3
```

## Constraints

- The tree has between `1` and `100` nodes.
- `-100 <= node.val <= 100`
- The tree is given in level order; `null` marks a missing child.
