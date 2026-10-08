# Hints

## Hint 1
Compare `nums` with a sorted copy. Which positions must lie inside the
subarray? That gives an O(n log n) answer. Can you avoid the sort?

## Hint 2
Scan left to right while tracking the maximum seen so far. Any element smaller
than that maximum is out of place, so the subarray must reach at least that
far to the right.

## Hint 3
The last such element is the right end. Symmetrically, scan right to left with
the running minimum: the last element larger than it (the leftmost one) is the
left end. If no element is out of place, the answer is `0`.
