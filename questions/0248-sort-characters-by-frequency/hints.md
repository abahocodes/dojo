# Hints

## Hint 1
Count how many times each character occurs. There are at most 62 distinct
characters, however long the string is.

## Hint 2
Sort the distinct characters, not the whole string, by the pair
(count descending, character code ascending).

## Hint 3
Build the answer by writing each character `count` times in that order. Use a
string builder or a join: repeated string concatenation can be quadratic.
