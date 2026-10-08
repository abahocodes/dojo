# Hints

## Hint 1
Searching for each word separately repeats a lot of work: words that share a
prefix explore the same paths over and over. Can you search for all of them at
once?

## Hint 2
Put every word into a trie. Then run one depth-first search from each cell,
walking the trie in step with the board: if the letters spelled so far are not
a prefix of any word, stop immediately.

## Hint 3
Store the full word on the trie node where it ends. When the DFS reaches such a
node, record the word and clear it so it isn't reported twice. Mark cells as
visited while they're on the current path, and remove trie branches that have
nothing left to find to keep later searches short.
