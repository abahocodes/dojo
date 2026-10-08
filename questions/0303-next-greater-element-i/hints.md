# Hints

## Hint 1
The answer for a value depends only on `nums2`. If you knew the next greater
element of every value in `nums2`, you could answer `nums1` with lookups.

## Hint 2
Scan `nums2` from left to right and keep the values that have not found their
next greater element yet. What order are those values in?

## Hint 3
They form a decreasing stack. When a new value `x` arrives, pop every smaller
value off the top. `x` is the next greater element of each one, so record it
in a map. Values left on the stack at the end map to `-1`.
