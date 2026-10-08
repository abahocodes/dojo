# Hints

## Hint 1
The ranked part only depends on how many times each letter of `order` occurs
in `s`, not on where.

## Hint 2
Count the letters of `s`. Walking through `order` and writing each letter as
many times as it was counted produces the first part.

## Hint 3
For the second part, walk `s` again and copy every letter that is not in
`order`. A 26-entry boolean table makes the membership test O(1).
