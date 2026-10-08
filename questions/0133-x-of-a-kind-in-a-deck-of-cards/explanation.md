# Approach: gcd of the counts

Count how many times each value appears. A pile size `X` works if and only
if it divides every count: the copies of a value are split into piles of
exactly `X`, and piles of different values never mix. The set of numbers
dividing every count is the set of divisors of `g = gcd(all counts)`. So a
valid `X >= 2` exists if and only if `g >= 2` (then `X = g` works).

```python
from collections import Counter
from math import gcd

def has_groups_size_x(deck):
    g = 0
    for c in Counter(deck).values():
        g = gcd(g, c)
    return g >= 2
```

Since values are below `10^4`, the other languages count into a fixed array.

## Complexity

- Time: O(n + V log n), where V is the number of distinct values.
- Space: O(V) for the counts.

## Pitfalls

- Checking only whether the smallest count is at least 2: counts 2 and 3 have
  no common pile size.
- Requiring every count to be equal: counts 2 and 4 work with `X = 2`.
- A single card (or any value appearing once) makes the answer `false`.
