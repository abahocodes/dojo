# Hints

## Hint 1
Where in the tree is the smallest value? Use the rule that every parent equals
the minimum of its children.

## Hint 2
The root holds the overall minimum. So you are looking for the smallest value
that is strictly greater than `root.val`.

## Hint 3
Traverse the tree. When a node's value is greater than `root.val`, it is a
candidate, and nothing below it can be smaller (its descendants are all at
least as large), so you can stop descending there. Keep the smallest
candidate, or `-1` if there is none.
