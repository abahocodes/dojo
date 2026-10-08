# Hints

## Hint 1
If the running total of the values is the same at two different nodes, then
the nodes strictly after the first and up to the second sum to `0`.

## Hint 2
Add a sentinel node with value `0` before the head so that a zero-sum run
starting at the head is handled like any other. Record, for each running total,
the **last** node at which it occurs.

## Hint 3
Make a second pass from the sentinel with a fresh running total. At each node,
set `node.next = last[total].next` and move on to `node.next`. Everything
between two equal totals disappears in one splice.
