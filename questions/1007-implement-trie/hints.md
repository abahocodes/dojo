# Hints

## Hint 1
Storing every word in a list and scanning it costs time proportional to all
stored text per query. Words that share a beginning should share storage for
that beginning.

## Hint 2
Make a tree where each node stands for a prefix, and each edge adds one
letter. The root is the empty prefix. Walking down the letters of a string
either reaches a node or falls off the tree.

## Hint 3
Give each node up to 26 children (an array or a map) and a flag `is_end`
marking that a whole word ends there. `insert` walks and creates missing
children, then sets the flag. `startsWith` succeeds if the walk reaches a
node; `search` additionally needs that node's flag to be set.
