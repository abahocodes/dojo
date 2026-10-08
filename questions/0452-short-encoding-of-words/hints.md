# Hints

## Hint 1
In the best reference string each chunk is a whole word, and the words that do
not need their own chunk are exactly the ones that end some other word.

## Hint 2
So the answer is the sum of `len(w) + 1` over the distinct words that are not
a proper suffix of any other word. How can you find those quickly when words
are at most 7 letters long?

## Hint 3
Put every word into a set. For each word, remove each of its proper suffixes
`w[1:], w[2:], ...` from the set. What remains are the chunk words. (A trie of
reversed words works too: the chunk words are its leaves.)
