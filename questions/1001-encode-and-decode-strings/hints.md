# Hints

## Hint 1
Joining with a separator like `","` breaks as soon as a string contains a
comma, and no single character is safe because the strings may hold anything.
Escaping can work, but there is a simpler idea.

## Hint 2
If `decode` knew how long the next string is, it could just take that many
characters, no matter what they are. How can you tell it the length?

## Hint 3
Write each string as its length, a marker such as `#`, then the string itself:
`["ab", "#"]` becomes `"2#ab1##"`. To decode, read digits up to the first `#`,
take exactly that many characters after it, and repeat from where you stopped.
The marker only ever has to be found right after a run of digits, so `#`
inside the data is harmless.
