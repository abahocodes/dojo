# Hints

## Hint 1
Comparing every element with every other element works but is O(n²). What
would you need to remember about the values you have already walked past?

## Hint 2
If the list were sorted, any repeated values would sit right next to each
other. That gives an O(n log n) answer. Can you avoid the sort?

## Hint 3
Keep a set of values seen so far. For each value, if it is already in the set
you have found a duplicate; otherwise add it. If the loop finishes, there are
none.
