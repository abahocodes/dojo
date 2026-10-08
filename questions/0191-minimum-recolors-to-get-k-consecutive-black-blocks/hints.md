# Hints

## Hint 1
Fix where the run of `k` black blocks will be. How many operations does that
particular choice cost?

## Hint 2
A chosen window of length `k` costs exactly the number of `'W'` characters
inside it. The answer is the minimum over all windows.

## Hint 3
Slide a window of length `k` across the string, keeping a running count of
whites: add one when a `'W'` enters, subtract one when a `'W'` leaves.
