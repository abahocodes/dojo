# Hints

## Hint 1
The cycle is the tree path from `a` to `b` plus the new edge. So the answer
is `dist(a, b) + 1`. How do you find the tree distance quickly?

## Hint 2
In heap labelling the parent of `v` is `v // 2`. A node's depth is
determined by its number of binary digits, so a larger label is never
shallower than a smaller one.

## Hint 3
While `a != b`, replace the larger of the two by its half and count a step.
They meet at the lowest common ancestor; the number of steps is the
distance. Each query takes at most about `2n` steps.
