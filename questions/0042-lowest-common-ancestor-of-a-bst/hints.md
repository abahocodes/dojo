# Hints

## Hint 1
You could find the root-to-node path for both values and compare them, but the
search-tree ordering lets you do much less work.

## Hint 2
Start at the root. If both values are smaller than the current node, where must
their common ancestor be? If both are larger? What if they fall on different
sides, or one of them equals the current node?

## Hint 3
Walk down from the root: go left while both `p` and `q` are smaller than the
node, go right while both are larger. The first node where they are not on the
same side (or where one of them equals the node) is the answer.
