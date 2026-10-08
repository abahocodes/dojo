# Hints

## Hint 1
Process the string one run at a time. Where does the current run end?

## Hint 2
Keep an index `i` at the start of a run and advance a second index `j` while
`chars[j] == chars[i]`. The run length is `j - i`.

## Hint 3
Append the character, append `str(j - i)` only when it is at least 2, then
continue from `i = j`. Build the answer in a list or string builder rather
than by repeated string concatenation.
