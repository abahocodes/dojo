# Hints

## Hint 1
A word is buildable exactly when all of its prefixes are words too. Which data
structure lays words out by their prefixes?

## Hint 2
Insert every word into a trie and mark the nodes where words end. A word is
buildable if every node on the path from the root to it is marked.

## Hint 3
Traverse the trie (DFS or BFS), but only step into a child if that child is
marked as a word end. Every node you reach this way is a buildable word: keep
the longest, breaking ties by the alphabetically smaller word.
