# Hints

## Hint 1
Comparing every pair of words is O(n²) comparisons. Instead, can you compute,
for each word, a value that is the same for all of its rearrangements and
different for everything else?

## Hint 2
Sorting a word's letters gives such a value: `"tops"` and `"stop"` both become
`"opst"`. Use it as a key in a hash map from key to list of words.

## Hint 3
To avoid sorting each word, count its letters instead: a tuple of 26 counts
(or a string built from it) is also a valid key. Append each word to
`groups[key]`, then return the map's values.
