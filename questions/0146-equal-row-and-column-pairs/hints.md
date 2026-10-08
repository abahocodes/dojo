# Hints

## Hint 1
Comparing every row with every column element by element costs `O(n^3)`.
Can you compare a whole row with a whole column in (expected) constant time?

## Hint 2
Turn every row into a hashable key (a tuple, a string with separators, a
list) and count how many times each key occurs.

## Hint 3
Build the same key for every column and add the count of matching rows. The
sum over all columns is the answer.
