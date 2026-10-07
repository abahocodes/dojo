# Hints

## Hint 1
Which traversal naturally visits nodes in order of their distance from the
root: depth-first or breadth-first?

## Hint 2
A queue gives you breadth-first order, but you also need to know where one
level ends and the next begins. What does the queue's size tell you at the
moment you start a new level?

## Hint 3
Loop while the queue is non-empty. Record `size = len(queue)`, pop exactly
`size` nodes into a fresh list (pushing their left then right children), and
append that list to the result.
