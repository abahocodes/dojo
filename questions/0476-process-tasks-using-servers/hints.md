# Hints

## Hint 1
Two things change over time: which servers are free, and when the busy ones
finish. You repeatedly need "the best free server" and "the server that
finishes first". Which data structure answers "smallest" quickly?

## Hint 2
Keep two min-heaps: free servers keyed by `(weight, index)`, and busy servers
keyed by `(free time, weight, index)`. Before assigning a task at time `t`,
move every busy server with free time `<= t` into the free heap.

## Hint 3
Track a current time that never moves backwards: for task `j` it is
`max(time, j)`. If the free heap is still empty, jump the time forward to the
top of the busy heap, release everything finishing then, and assign. No
second-by-second loop is needed.
