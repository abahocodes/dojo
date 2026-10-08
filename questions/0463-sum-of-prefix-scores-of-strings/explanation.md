# Approach: trie with pass-through counters

Each non-empty prefix `t` corresponds to one trie node, and the words that
start with `t` are exactly the words whose insertion path passes through that
node. So:

1. Insert every word; on each node you step into, increment its `count`.
   After all insertions, `count[node]` is the score of that node's prefix.
2. For each word, walk its path again and sum the counts. The path visits each
   of the word's non-empty prefixes exactly once.

```python
def sum_prefix_scores(words):
    trie, count = [{}], [0]
    for w in words:
        node = 0
        for ch in w:
            if ch not in trie[node]:
                trie[node][ch] = len(trie)
                trie.append({})
                count.append(0)
            node = trie[node][ch]
            count[node] += 1

    result = []
    for w in words:
        node, total = 0, 0
        for ch in w:
            node = trie[node][ch]
            total += count[node]
        result.append(total)
    return result
```

## Complexity

- Time: O(S), where `S` is the total number of characters (two passes).
- Space: O(S · σ) for the trie (σ = 26 with array children, O(S) with maps).

## Pitfalls

- Don't count the root: the empty prefix is excluded.
- Duplicate words each count separately, and the counters handle that
  automatically.
- Counting prefixes in a hash map of strings also works, but builds O(L²)
  characters of keys per word; that's 5 · 10^8 characters at the limits.
- Totals stay below `1000 · 1000 = 10^6`, so 32-bit integers suffice.
