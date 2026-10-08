# Hints

## Hint 1
Comparing every prefix of every word against every other word is cubic. The
score of a prefix depends only on the prefix itself, so many words share the
same work. Where do words with a common prefix "meet"?

## Hint 2
In a trie, every word with prefix `t` passes through the node for `t`. If each
node remembers how many words passed through it during insertion, that number
is exactly the score of the corresponding prefix.

## Hint 3
Insert every word, incrementing a counter on each node you step into (not the
root). Then walk each word again from the root and add up the counters along
its path: that sum is its answer.
