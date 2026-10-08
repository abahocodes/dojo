# Hints

## Hint 1
If you generate the pairs by looping over the start `i` in increasing order,
and for each `i` over the end `j` in increasing order, they come out already
sorted.

## Hint 2
Checking every slice `text[i..j]` against a set of the words works, but it
builds a new string per slice. Can you extend the match from `i` one character
at a time and stop as soon as no word can continue?

## Hint 3
Put the words in a trie and mark the nodes where a word ends. From each start
`i`, walk the trie along `text[i], text[i+1], ...`. Record `[i, j]` whenever
the node is marked, and stop when the next character has no child.
