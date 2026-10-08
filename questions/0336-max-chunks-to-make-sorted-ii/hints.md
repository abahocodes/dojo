# Hints

## Hint 1
When is a cut between positions `i` and `i + 1` allowed on its own? Everything
on the left must be able to stay on the left in the sorted order.

## Hint 2
A cut after position `i` is safe exactly when `max(arr[0..i]) <= min(arr[i+1..])`.
And the safe cuts can all be made at once, so the answer is one plus the number
of safe cuts. Prefix maxima and suffix minima give an O(n) solution.

## Hint 3
A stack also works in one pass: keep the maximum of every chunk so far
(non-decreasing from bottom to top). A new value `x` smaller than the top
must merge with every chunk whose maximum exceeds `x`; pop them, then push back
the largest of the popped maxima. The stack size is the answer.
