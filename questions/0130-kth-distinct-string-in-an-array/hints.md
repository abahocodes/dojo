# Hints

## Hint 1
Whether a string is distinct depends on the whole array, so you cannot
decide it on the first sighting.

## Hint 2
Make two passes: one to count how many times each string occurs, one to
walk the array in order.

## Hint 3
In the second pass, every string whose count is 1 decrements `k`; return the
string that brings `k` to 0. If the pass ends first, return `""`.
