# Hints

## Hint 1
Rotating by `n` gives back the original array, so only `k mod n` matters.

## Hint 2
After the rotation, the last `k` elements sit at the front, in their
original order, followed by the first `n - k` elements. Can reversing parts
of the array produce that layout?

## Hint 3
Reverse the whole array, then reverse the first `k` elements, then reverse
the remaining `n - k`. Each reversal is a two-pointer swap loop.
