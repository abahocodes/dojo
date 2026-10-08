# Hints

## Hint 1
With at most 50 words, you can afford to look at every pair `i < j`
directly.

## Hint 2
For a pair, `words[i]` must fit inside `words[j]`, match its first
`len(words[i])` characters, and also match its last `len(words[i])`
characters. The two checks may overlap in the middle of `words[j]`; that is
fine.

## Hint 3
Loop `j` over the list and `i` over the indices before it, and count the pairs
where `words[j].startswith(words[i]) and words[j].endswith(words[i])`.
