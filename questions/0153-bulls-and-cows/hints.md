# Hints

## Hint 1
Bulls are easy: compare the strings position by position.

## Hint 2
For cows, only the non-bull positions matter, and order no longer matters
there. Count how often each digit occurs among the leftover characters of
each string.

## Hint 3
Cows = sum over digits of `min(left_secret[d], left_guess[d])`. Or do it in
one pass with a single array: `secret` digits add 1, `guess` digits subtract
1, and a cow is found whenever the update meets a count of the opposite sign.
