# Hints

## Hint 1
Whether a character is unique depends on the whole string, so you can't decide
in the same pass that first sees it.

## Hint 2
Make one pass to count how often each letter appears.

## Hint 3
Make a second pass in index order and return the first index whose letter has
a count of 1.
