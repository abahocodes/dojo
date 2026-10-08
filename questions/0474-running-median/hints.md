# Hints

## Hint 1
Re-sorting the prefix after every arrival costs `O(n² log n)`; even inserting
into a sorted list is `O(n²)` because of shifting. You only ever need the one
or two values in the middle.

## Hint 2
Split the numbers seen so far into a lower half and an upper half. The median
depends only on the largest value of the lower half and the smallest value of
the upper half. Which data structure gives fast access to those?

## Hint 3
Keep a max-heap `low` and a min-heap `high`, with every element of `low` at
most every element of `high`, and `len(low)` equal to `len(high)` or one more.
Insert into `low` if the new value is at most `low`'s top, otherwise into
`high`, then move one top across if the sizes drift apart. The median is
`low`'s top (odd count) or the average of both tops (even count).
