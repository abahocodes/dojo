# Approach: Hierholzer's algorithm with sorted destinations

Treat airports as nodes and tickets as directed edges. A trip that uses
every ticket once is an **Eulerian path** starting at `JFK`.

Greedily flying to the smallest destination is not enough: it can walk into
a dead end while tickets are still unused (Example 2). Hierholzer's
algorithm fixes this without backtracking:

1. Walk from `JFK`, always taking the smallest unused ticket out of the
   current airport.
2. When you're at an airport with no tickets left, you're stuck. Any valid
   route must end its remaining part here, so this airport is the **last**
   one not yet placed. Append it to the output and step back to the
   previous airport, then keep walking from there.
3. The output is built from the end of the trip backwards; reverse it.

A detour that got stuck early is placed after everything that is later
spliced in before it, which is exactly how the dead end in Example 2 ends
up last. Taking the smallest destination at each step makes the final route
the lexicographically smallest Eulerian path.

```python
from collections import defaultdict

def find_itinerary(tickets):
    graph = defaultdict(list)
    for src, dst in sorted(tickets, reverse=True):
        graph[src].append(dst)   # descending, so pop() gives the smallest

    route, stack = [], ["JFK"]
    while stack:
        if graph[stack[-1]]:
            stack.append(graph[stack[-1]].pop())
        else:
            route.append(stack.pop())
    return route[::-1]
```

## Complexity

- Time: O(E log E) for sorting `E` tickets; the walk itself is O(E).
- Space: O(E) for the adjacency lists, stack and route.

## Pitfalls

- Plain greedy (always fly to the smallest airport and never step back)
  fails when the smallest choice is a dead end.
- Backtracking DFS (try destinations in order, undo on failure) is correct
  but can be exponential on graphs with many repeated tickets.
- Duplicate tickets are separate edges. Store destinations in a list or
  multiset, not a set.
- Remove each ticket as you use it (pop it), so it is never reused.
- A recursive version recurses once per ticket; with 300 tickets that's
  fine, but the iterative stack avoids the question entirely.
