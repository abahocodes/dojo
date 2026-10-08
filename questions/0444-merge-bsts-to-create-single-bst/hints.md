# Hints

## Hint 1
Every tree except one must be merged into another tree, so every root except
one must appear as a leaf somewhere. Which root becomes the root of the final
tree?

## Hint 2
The final root is the unique root whose value is not a leaf value in any tree.
If there are zero or several such roots, the answer is empty. Keep the other
trees in a hash map keyed by root value.

## Hint 3
Walk down from the final root with a stack of `(node, lo, hi)` bounds. At a
leaf whose value is in the map, pop that tree from the map and adopt its
children. Fail if a node falls outside `(lo, hi)`. At the end, every tree must
have been used: the map must be empty.
