## Hint 1
If every number were positive, the answer would be the product of the three
largest. What changes when negatives are allowed?

## Hint 2
Two negatives multiply to a positive. So the best product is either the three
largest values, or the two smallest (most negative) values times the largest.

## Hint 3
Sorting gives both candidates in `O(n log n)`. To do it in one pass, track the
three largest and the two smallest values as you scan.
