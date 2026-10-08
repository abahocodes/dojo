# Hints

## Hint 1
Wrapping around is easiest with indices taken modulo `n`: the element after
index `n - 1` is `(n - 1 + 1) % n = 0`.

## Hint 2
Summing `|k|` elements for each of the `n` positions works, but neighbouring
positions share almost all of their summed elements. Can you slide a window
around the ring instead?

## Hint 3
For `k > 0`, the window for index `0` is indices `1..k`. For `k < 0`, it is
`n - |k| .. n - 1`. Compute that first sum, then for each next index add the
element that enters the window and subtract the one that leaves, all modulo
`n`. Write results into a new array so the original values stay intact.
