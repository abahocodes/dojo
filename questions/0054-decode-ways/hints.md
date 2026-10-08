# Hints

## Hint 1
Try every split recursively and the number of calls doubles with length.
But the number of decodings of a suffix doesn't depend on how you got there.

## Hint 2
Let `ways[i]` count decodings of the first `i` digits. The last piece is either
one digit (valid if it isn't `0`) or two digits (valid if they form `10`-`26`).

## Hint 3
`ways[i] = (s[i-1] != '0' ? ways[i-1] : 0) + (10 <= int(s[i-2:i]) <= 26 ? ways[i-2] : 0)`
with `ways[0] = 1`. Only the last two values are needed.
