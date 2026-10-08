# Approach: count, then rank by a composite key

1. Count occurrences with a hash map.
2. Rank the distinct words by the key `(-count, word)`: higher counts first,
   alphabetical order breaking ties.
3. Return the first `k`.

```python
from collections import Counter


def top_k_frequent_words(words, k):
    counts = Counter(words)
    ranked = sorted(counts, key=lambda w: (-counts[w], w))
    return ranked[:k]
```

**Heap variant:** with `m` distinct words, sorting costs `O(m log m)`. A heap
of size `k` brings that down to `O(m log k)`. The heap's top must be the word
that would be evicted first, the one with the smallest count and, among equal
counts, the alphabetically *largest*. Note that this inverts the string order
compared with the count order, which is the classic trap. Pop the survivors
and reverse them at the end. `heapq.nsmallest(k, counts, key=lambda w: (-counts[w], w))`
does all of this for you in Python.

## Complexity

- Time: `O(n + m log m)` for `n` words and `m` distinct words (`O(n + m log k)`
  with the heap variant). Word length is at most 10, so comparisons are cheap.
- Space: `O(m)` for the counts.

## Pitfalls

- Breaking ties by first appearance or by hash-map iteration order instead of
  alphabetically.
- In a size-`k` min-heap, using the same direction for both parts of the key:
  among equal counts, the alphabetically *later* word must be evicted first.
- Returning the heap's contents without sorting them into final order.
