# Hints

## Hint 1
Each server receives the alert at the length of the shortest path from `k` to
it. Once you know all those shortest times, what is the answer?

## Hint 2
The answer is the largest shortest-path distance, or `-1` if some server is
unreachable. The links have non-negative weights, so a classic single-source
shortest path algorithm applies.

## Hint 3
Use Dijkstra's algorithm with a min-heap of `(time, server)`. Pop the smallest
time; if that server is already settled, skip it, otherwise record its time
and push each neighbour with `time + w`. When the heap is empty, check that
every server got a time and return the maximum.
