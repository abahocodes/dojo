# Hints

## Hint 1
Suffixes become prefixes if you reverse the words. Which data structure finds
the longest common prefix of a query with a whole set of words?

## Hint 2
Insert every reversed container word into a trie. The deepest trie node a
reversed query can reach is its longest common suffix with the container.

## Hint 3
The query only needs one index per node, so precompute it: every node stores
the best container index (shortest, then smallest index) among the words that
pass through it, and the root stores the best overall. Answer each query with
the stored index of the deepest node reached.
