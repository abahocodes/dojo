# Hints

## Hint 1
Each query can be checked on its own. The pattern must appear in the query as
a subsequence. What else must be true about the leftover characters?

## Hint 2
Every query character that is not used to match a pattern character was
"inserted", so it must be lowercase.

## Hint 3
Walk the query with a pointer `j` into the pattern. If the character equals
`pattern[j]`, advance `j`; otherwise, if it is uppercase, fail. At the end,
the query matches only if `j` reached the end of the pattern.
