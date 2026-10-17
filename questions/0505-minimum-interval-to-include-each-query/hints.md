# Hints

## Hint 1
Checking every range for every query is O(n * q), too slow at 10^5 each. The
answer for a query doesn't depend on the order queries are asked in, so you
are free to answer them in whatever order is convenient, then put the answers
back in place.

## Hint 2
Answer the queries from smallest to largest, and walk the ranges sorted by
their left end. When you reach query `q`, every range with `left <= q` has
been seen. Among those, you want the smallest one whose `right >= q`.

## Hint 3
Push each newly reached range into a min-heap keyed by size, storing its right
end too. Before reading the top, pop ranges whose right end is below `q`:
they can't contain `q`, and since later queries are even larger, they never
will. The top (if any) is the answer; otherwise it is `-1`.
