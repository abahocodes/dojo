# Hints

## Hint 1
The common prefix can never be longer than the shortest string, and it can only
shrink as you look at more strings.

## Hint 2
Start with the first string as your candidate prefix. Compare it with each
other string and cut it down to the part they agree on.

## Hint 3
For each next string, advance an index `i` while both strings have a character
at `i` and the characters match, then keep only `prefix[:i]`. Stop early once
the prefix is empty.
