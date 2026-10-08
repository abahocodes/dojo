You are given `head`, the first node of a singly linked list of integers.

Any run of one or more **consecutive** nodes whose values add up to `0` can be
cut out of the list. Keep cutting such runs until the list contains none, and
return the head of what is left (`None` if every node is removed).

Different cutting orders can leave different lists, so the answer is defined
by the following prefix-sum rule. Put a sentinel with value `0` in front of the
list and let the prefix sum of a node be the total of the values from the
sentinel up to and including that node. Walk from the sentinel, left to right:
at each node you keep, look up the **last** node in the whole list with the
same prefix sum and splice out everything after the current node up to and
including that last node. The next node you keep is the one that follows the
removed stretch.

## Example 1

```
head   = 3 -> 4 -> -4 -> 2 -> -2 -> 5
output = 3 -> 5
```

Prefix sums are `3, 7, 3, 5, 3, 8`. From the node `3` (prefix `3`) the last
node with prefix `3` is the `-2`, so `4, -4, 2, -2` are cut and `5` follows.

## Example 2

```
head   = 2 -> -1 -> 1 -> -2 -> 6 -> 1 -> -1
output = 6
```

`2, -1, 1, -2` sums to `0` and so does `1, -1`.

## Constraints

- The list has between `1` and `1000` nodes.
- `-1000 <= node.val <= 1000`
