# Hints

## Hint 1
Turn the question around: given a limit `x`, how many operations do you need
so that no bag holds more than `x` balls?

## Hint 2
A bag of `b` balls must end up as at least `ceil(b / x)` bags, and each split
adds one bag, so it needs `ceil(b / x) - 1 = (b - 1) // x` splits. Bags
are independent.

## Hint 3
The required operation count only decreases as `x` grows. Binary search the
smallest `x` in `[1, max(nums)]` with `sum((b - 1) // x) <= max_operations`,
using a 64-bit sum (or stopping early once it exceeds the budget).
