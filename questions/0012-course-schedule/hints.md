# Hints

## Hint 1
Draw the courses as nodes and each prerequisite as an arrow from `b` to `a`.
What shape in this graph makes finishing impossible?

## Hint 2
The courses can all be finished exactly when the directed graph has no cycle.
A course with no unmet prerequisites can always be taken right now. What
happens to the other courses once you take it?

## Hint 3
Count each course's unmet prerequisites (its in-degree). Put every course with
count `0` in a queue. Repeatedly take one, and decrement the count of each
course it unlocks, enqueuing any that drop to `0`. You succeed if you take all
`num_courses` courses.
