# Hints

## Hint 1
Plain Dijkstra finds the cheapest route, but it may use too many flights. What
extra piece of information does each partial trip need to carry?

## Hint 2
Think in rounds: after round `i`, know the cheapest price to reach every city
using at most `i` flights. How do you get round `i + 1` from round `i`?

## Hint 3
Bellman-Ford limited to `k + 1` rounds. In each round, start from a *copy* of
the previous prices and relax every flight using the previous round's values:
`next[to] = min(next[to], prev[from] + price)`. Reading from the copy ensures
a round adds at most one flight. After `k + 1` rounds, `prev[dst]` is the
answer (or `-1` if it is still infinite).
