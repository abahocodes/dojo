# Hints

## Hint 1
Break the sentence into its words. Then the problem is about one word at a
time.

## Hint 2
Walk the words in order, keeping a 1-based counter. Stop at the first word
that passes the prefix test, so later matches never override it.

## Hint 3
Split on single spaces, then return `index + 1` for the first word with
`word.startswith(search_word)`, or `-1` after the loop. Without splitting, you
can scan the characters, noting where each word starts.
