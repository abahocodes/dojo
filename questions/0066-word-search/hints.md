# Hints

## Hint 1
Every cell holding `word[0]` is a possible starting point. From there, the
next letter must sit in one of up to four neighbours.

## Hint 2
Explore with a depth-first search that tracks which letter of `word` you need
next. The search has to remember which cells the current path already uses,
and forget them again when it backs out.

## Hint 3
Write `dfs(r, c, i)`: fail if `(r, c)` is off the board or doesn't hold
`word[i]`; succeed if `i` is the last index. Otherwise mark the cell as used
(e.g. overwrite it with `"#"`), try the four neighbours with `i + 1`, then
restore the letter before returning.
