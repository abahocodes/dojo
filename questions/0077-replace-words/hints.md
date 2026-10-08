# Hints

## Hint 1
For each word you need the shortest root that is a prefix of it. Checking
every root against every word works, but how much of that comparison is
repeated?

## Hint 2
A trie stores all roots by their prefixes. Walking a word down the trie one
letter at a time visits exactly the roots that are prefixes of it, shortest
first.

## Hint 3
Insert every root and mark the node where it ends. For each word, walk the
trie: the first marked node you reach gives the answer (the word's first
`i + 1` letters). If you fall off the trie or run out of letters first, keep
the word unchanged.
