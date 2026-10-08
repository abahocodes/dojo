# Hints

## Hint 1
Counting bits of each number on its own works. To do better, relate the
count for `i` to the count for some smaller number you have already filled in.

## Hint 2
Shifting `i` right by one drops its lowest bit. How does the bit count of
`i >> 1` compare to the bit count of `i`?

## Hint 3
`bits[i] = bits[i >> 1] + (i & 1)`. Fill the list from left to right, starting
with `bits[0] = 0`.
