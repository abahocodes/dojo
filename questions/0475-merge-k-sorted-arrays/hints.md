# Hints

## Hint 1
Concatenating and sorting works in `O(N log N)` for `N` total elements, but it
ignores the fact that every input is already sorted. How do you merge just two
sorted arrays?

## Hint 2
To merge `k` arrays, at each step you need the smallest among the `k` current
front elements. Scanning all fronts costs `O(k)` per element. Which data
structure returns the minimum of a changing set faster?

## Hint 3
Keep a min-heap of `(value, array index, position)` holding the front of every
non-empty array. Pop the smallest, append its value, and push the next element
from the same array if there is one. Skip empty arrays when seeding the heap.
