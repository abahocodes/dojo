# Hints

## Hint 1
Without dots this is exactly a trie lookup. Build the same tree of letters,
with a flag on the nodes where a whole word ends.

## Hint 2
A `.` doesn't tell you which child to follow, so you have to try them all.
That makes `search` a depth-first search over the trie instead of a single
walk.

## Hint 3
Write `match(node, i)`: if `i == len(word)` return `node.is_end`. If
`word[i]` is a letter, recurse into that one child (if it exists). If it is
`.`, return `true` as soon as `match(child, i + 1)` is true for any child.
Start with `match(root, 0)`.
