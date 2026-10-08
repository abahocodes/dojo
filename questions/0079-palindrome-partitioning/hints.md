# Hints

## Hint 1
Every way of cutting the string starts with a first piece. Which first pieces
are allowed, and what is left to solve after choosing one?

## Hint 2
Backtracking: from position `start`, try every end position whose piece
`s[start..end]` is a palindrome, add it to the current list, recurse from
`end + 1`, then remove it. When `start` reaches the end of the string, record
a copy of the current list.

## Hint 3
Checking palindromes over and over is wasteful. Precompute a table where
`pal[i][j]` says whether `s[i..j]` is a palindrome: it is when
`s[i] == s[j]` and the inside `s[i+1..j-1]` is a palindrome (or empty).
