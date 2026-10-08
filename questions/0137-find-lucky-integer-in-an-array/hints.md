# Hints

## Hint 1
You need the frequency of every value. How would you count them?

## Hint 2
Values are between 1 and 500, so an array of 501 counters works as well as a
hash map.

## Hint 3
After counting, check values from 500 down to 1 and return the first `v`
with `count[v] == v`. If none matches, return `-1`.
