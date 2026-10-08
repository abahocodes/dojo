# Approach: one combined count

"Exactly once in one sentence and absent from the other" is the same as
"exactly once in both sentences combined". So split both sentences into
words, count all of them in one hash map, and keep the words with count 1.

```python
from collections import Counter

def uncommon_from_sentences(s1, s2):
    counts = Counter(s1.split() + s2.split())
    return [w for w, c in counts.items() if c == 1]
```

## Complexity

- Time: O(n) in the total length of the sentences.
- Space: O(n) for the map.

## Pitfalls

- Computing the words of `s1` missing from `s2` (and vice versa) with sets:
  that keeps words repeated inside a single sentence, which are not uncommon.
- Returning `null` / `nil` when there are no uncommon words; return an
  empty list.
