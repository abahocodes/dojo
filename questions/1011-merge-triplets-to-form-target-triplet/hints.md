# Hints

## Hint 1
A merge takes maximums, so values only ever grow. What does that tell you
about a triplet that has some value larger than the target in the same
position?

## Hint 2
Such a triplet can never be part of a merge that produces the target: it
would push that position past the target forever. Throw those away. Every
remaining triplet is "safe": merging it in never overshoots.

## Hint 3
Merge all safe triplets together (take the element-wise maximum over them).
The result never exceeds the target, and it equals the target exactly when
each of the three target values is matched by at least one safe triplet.
