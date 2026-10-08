# Hints

## Hint 1
This is the mirror image of "does the word start with a prefix": here `s` is
the long string and each word is the candidate prefix.

## Hint 2
A word longer than `s` can never be its prefix. Otherwise compare the word with
the first `len(word)` characters of `s`.

## Hint 3
Count the words for which `s.startswith(word)` is true. Duplicates are separate
entries, so do not deduplicate the list first.
