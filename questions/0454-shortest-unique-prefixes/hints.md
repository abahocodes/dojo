# Hints

## Hint 1
For one word, try prefixes of length 1, 2, 3, ... and stop at the first one
that no other word starts with. How do you answer "how many words start with
this prefix?" quickly?

## Hint 2
A trie answers it: every trie node is a prefix. Store at each node how many
words pass through it.

## Hint 3
Insert all words, incrementing the counter of every node on the path. Then
walk each word again and stop at the first node whose counter is 1; the path
so far is the answer.
