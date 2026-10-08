# Hints

## Hint 1
Work out what `k = 1` allows: the only thing you can ever do is move the first
character to the back. Which strings can you reach that way?

## Hint 2
With `k = 1` the reachable strings are exactly the rotations of `s`, so the
answer is the smallest rotation. Now think about `k = 2`: can you swap two
adjacent characters somewhere in the string?

## Hint 3
With `k >= 2` you can rotate freely and, by holding back one character while
cycling the rest, swap any adjacent pair. Adjacent swaps generate every
permutation, so the answer is simply the sorted string.
