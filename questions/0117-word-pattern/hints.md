# Hints

## Hint 1
Split `s` into words first. If the number of words differs from the length of
the pattern, you are done.

## Hint 2
Now pair `pattern[i]` with `words[i]`. A letter seen before must pair with the
same word as last time.

## Hint 3
That alone lets two letters share a word. Keep two maps, letter -> word and
word -> letter, and reject any pair that disagrees with either.
