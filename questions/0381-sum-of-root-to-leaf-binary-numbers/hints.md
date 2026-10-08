# Hints

## Hint 1
You never need to build the binary strings. If the path so far spells the
number `p`, what number does it spell after appending one more bit `b`?

## Hint 2
Appending a bit is a left shift plus that bit: `p * 2 + b` (or
`(p << 1) | b`). Carry this running value down the tree as you traverse it.

## Hint 3
Do a depth-first traversal whose stack holds `(node, value_above_node)`.
At each node compute `value = value_above * 2 + node.val`. At a leaf add
`value` to the total; otherwise push each existing child with `value`.
