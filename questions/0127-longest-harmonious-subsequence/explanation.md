# Approach: count, then pair each value with its successor

Max minus min equals 1 means the subsequence uses exactly two values, `x`
and `x + 1`, each at least once. Order is irrelevant for max and min, and
taking every copy of both values only helps, so the best subsequence built
on `x` has length `count[x] + count[x + 1]`. Count with a hash map and try
every `x` whose successor is present.

```python
from collections import Counter


def find_lhs(nums):
    count = Counter(nums)
    best = 0
    for x, c in count.items():
        if x + 1 in count:
            best = max(best, c + count[x + 1])
    return best
```

Sorting and sweeping pairs of adjacent runs also works, in O(n log n).

## Complexity

- Time: O(n) expected.
- Space: O(m) for the counts, `m` distinct values.

## Pitfalls

- Counting a single repeated value: `[7, 7, 7]` has max minus min `0`, so it
  is not harmonious and the answer is `0`.
- Checking `x - 1` and `x + 1` both is fine but redundant; each pair is
  found from its smaller value.
- In Java and C++, computing `x + 1` for `x = 2^31 - 1` overflows `int`.
  Here `|nums[i]| <= 10^9`, but the reference Java and C++ solutions use
  64-bit keys anyway.
