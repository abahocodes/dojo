# Hints

## Hint 1
Checking every pair of strings is quadratic. How could you find a string's
index in `list1` in O(1)?

## Hint 2
Map every string of `list1` to its index. Then walk `list2`: every string
found in the map is common, and its index sum is immediate.

## Hint 3
Track the smallest sum so far. A smaller sum replaces the result list with
the current string; an equal sum appends to it.
