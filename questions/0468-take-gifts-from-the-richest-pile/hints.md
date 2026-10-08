# Hints

## Hint 1
Simulating second by second is fine; the only question is how quickly you can
find the largest pile each time. A linear scan per second costs `O(n · k)`.

## Hint 2
You repeatedly need "the current maximum", and you change one value at a time.
That is exactly what a max-heap (priority queue) is for.

## Hint 3
Put every pile into a max-heap. For `k` rounds, pop the top `x` and push back
`floor(sqrt(x))`. Finally sum what is left in the heap, using a 64-bit total
because up to `10^4` piles of `10^9` gifts overflow a 32-bit integer.
