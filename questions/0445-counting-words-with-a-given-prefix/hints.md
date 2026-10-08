# Hints

## Hint 1
Each word can be checked on its own. What has to be true about a word that
starts with `pref`, before you even look at its letters?

## Hint 2
A word shorter than `pref` can never match. Otherwise compare the first
`len(pref)` characters of the word with `pref`.

## Hint 3
Most languages have a built-in prefix test (`startswith`, `startsWith`,
`strings.HasPrefix`). Loop over the words and count the ones that pass it.
