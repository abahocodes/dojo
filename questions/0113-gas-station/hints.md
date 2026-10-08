# Hints

## Hint 1
Trying every start and simulating the trip is O(n²). First: if the total fuel
is less than the total cost, can any start work?

## Hint 2
Suppose you start at `s` and your tank first goes negative while leaving
station `j`. Could any station between `s` and `j` have been a better start?
(Every station you passed was reached with a tank of at least `0`.)

## Hint 3
Walk once, keeping a running `tank`. Whenever it goes negative after station
`i`, discard every start up to `i`: reset `tank` to `0` and set the candidate
start to `i + 1`. If the overall total is non-negative, the final candidate is
the answer.
