# Hints

## Hint 1
Think of each word as a node, with an edge between two words that differ in
exactly one position. What are you looking for in that graph, and which search
finds it?

## Hint 2
You want the shortest path from `begin_word` to `end_word` in an unweighted
graph, which is a breadth-first search. The hard part is finding a word's
neighbours quickly: comparing it with every other word costs O(N · L) per word.

## Hint 3
Generate neighbours instead of searching for them. For each position, try all
26 letters and check whether the result is in a set of the remaining words.
Remove a word from the set as soon as you enqueue it so it is never visited
twice. Return the BFS depth when you produce `end_word`.
