# Hints

## Hint 1
You need every level's nodes grouped together. Which traversal hands you the
tree one level at a time?

## Hint 2
In a breadth-first search, the queue holds exactly one full level at the
moment a new level starts. Its size is that level's node count.

## Hint 3
For each level, add up the values of the `size` nodes you pop (pushing their
children), then append `total / size`. Keep the total in a 64-bit integer or a
float: two values near `2^31` already overflow a 32-bit sum.
