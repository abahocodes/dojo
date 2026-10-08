# Hints

## Hint 1
A valid order is a topological order of the graph with an arrow `b -> a` for
each prerequisite. Kahn's algorithm builds one by repeatedly taking a course
with no unmet prerequisites. Where in that algorithm do you get a choice?

## Hint 2
Whenever several courses are ready at once, you may take any of them. To make
the order lexicographically smallest, which one should you always pick?

## Hint 3
Always take the smallest ready course. Keep the ready courses in a min-heap
instead of a queue: pop the smallest, append it to the order, decrement the
in-degree of each course it unlocks, and push any that reach `0`. If the order
ends up shorter than `num_courses`, a cycle blocked the rest: return `[]`.
