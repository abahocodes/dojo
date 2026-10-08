# Hints

## Hint 1
Use one index `i` that walks both strings at the same time.

## Hint 2
At each step, append `word1[i]` if it exists, then `word2[i]` if it exists.

## Hint 3
Keep going while `i` is inside at least one of the strings; that handles the
leftover tail automatically. Collect characters in a list or builder and join
once at the end.
