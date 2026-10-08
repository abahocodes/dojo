# Hints

## Hint 1
In a preorder walk, once you move into the right subtree of some node `x`,
every later value must be greater than `x`. What does that say about a lower
bound that only ever grows?

## Hint 2
While values keep decreasing you are descending left. A value larger than the
previous one means you finished some left subtrees and moved right: which
ancestor did you move right from?

## Hint 3
Keep a stack of values that is decreasing from bottom to top, and a lower
bound `low` (initially minus infinity). For each `x`: if `x < low`, return
false. Pop every stack value smaller than `x`, setting `low` to the last value
popped (that is the node whose right subtree you entered). Push `x`. If the
loop finishes, return true.
