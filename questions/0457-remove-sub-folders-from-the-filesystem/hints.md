# Hints

## Hint 1
A folder is removed if any of its ancestors (the path cut at one of its `/`
characters) is in the list. A set of all paths lets you test that directly.

## Hint 2
The output must be sorted anyway. After sorting, where do the subfolders of a
folder `P` end up relative to `P`?

## Hint 3
They come right after `P` (since `/` sorts before every letter). Scan the
sorted list, remembering the last kept folder; skip a path if it starts with
`last + "/"`, otherwise keep it and make it the new `last`.
