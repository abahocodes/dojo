# Hints

## Hint 1
Comparing every pair is up to 2.5 billion comparisons. Instead, think about
which prefixes of `arr1`'s numbers exist at all.

## Hint 2
A decimal prefix of a number is just the number with some trailing digits
chopped off: `x, x / 10, x / 100, ...` (integer division). Each number has at
most 9 of them.

## Hint 3
Store every prefix of every `x` in a hash set (or a digit trie). For each `y`,
chop digits off the end until the value is in the set; the digit count of what
remains is that `y`'s best match.
