# Hints

## Hint 1
A counting map solves it in O(n) time but O(n) memory. What operation makes
two equal things cancel each other out?

## Hint 2
Exclusive or (XOR) has `x ^ x == 0` and `x ^ 0 == x`, and it does not care
about order.

## Hint 3
XOR every number into one accumulator that starts at `0`. Each pair cancels,
so only the lonely value is left.
