# Hints

## Hint 1
Airports are nodes and tickets are directed edges. You need a walk from
`JFK` that uses every edge exactly once: an Eulerian path. Among all of
them, you want the lexicographically smallest.

## Hint 2
Always flying to the smallest destination fails (see Example 2), and
backtracking can blow up. Hierholzer's algorithm builds an Eulerian path in
linear time: keep walking along unused edges; when you're stuck at an
airport with no tickets left, that airport is final for its position in the
route, so emit it and step back.

## Hint 3
Sort each airport's destinations and always take the smallest unused one.
Use a stack: look at the top airport; if it has an unused ticket, push its
smallest destination; otherwise pop it onto the result. The result comes
out in reverse; flip it at the end. Dead ends (like `KUL` in Example 2) get
emitted first, so they land at the end of the route.
