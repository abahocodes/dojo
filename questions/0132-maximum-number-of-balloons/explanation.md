# Approach: count letters, take the bottleneck

Spelling `"balloon"` once consumes `b`, `a`, `n` once each and `l`, `o` twice
each. Count every letter in `text`. The letter `b` alone allows `count[b]`
copies, `l` allows `count[l] // 2` copies, and so on. The real answer is
limited by the scarcest requirement, so it is the minimum over the five
letters of `count[letter] // need[letter]`.

```python
from collections import Counter

def max_number_of_balloons(text):
    have = Counter(text)
    need = Counter("balloon")
    return min(have[c] // k for c, k in need.items())
```

The other languages use a 26-entry array instead of a hash map.

## Complexity

- Time: O(n) to count the letters.
- Space: O(1): 26 counters.

## Pitfalls

- Forgetting that `l` and `o` are needed twice; dividing them by 2 is the
  whole trick.
- Returning the count of the rarest letter in `text` rather than the rarest
  letter of `"balloon"`.
- A missing letter (count 0) must give 0, not be skipped.
