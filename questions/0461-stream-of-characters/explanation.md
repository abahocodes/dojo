# Approach: trie of reversed words

A word `w` is a suffix of the received text exactly when reading the text
**backwards** from the newest character spells `reversed(w)` first. So insert
every word reversed into a trie and mark the node where each one ends.

After character `i` arrives, walk the trie from the root with
`stream[i], stream[i-1], stream[i-2], ...`:

- reaching a marked node means some word is a suffix → `true`;
- a missing child means no word can match from here on → `false`;
- no walk needs more than `L = max(len(word))` steps.

```python
def stream_checker(words, stream):
    trie, end = [{}], [False]
    longest = max(len(w) for w in words)
    for w in words:
        node = 0
        for ch in reversed(w):
            if ch not in trie[node]:
                trie[node][ch] = len(trie)
                trie.append({})
                end.append(False)
            node = trie[node][ch]
        end[node] = True

    result = []
    for i in range(len(stream)):
        node, found = 0, False
        for j in range(i, max(-1, i - longest), -1):
            node = trie[node].get(stream[j])
            if node is None:
                break
            if end[node]:
                found = True
                break
        result.append(found)
    return result
```

In the original "design" form of this problem, the stream is unbounded, so you
would keep only the last `L` characters in a buffer.

## Complexity

- Time: O(W + n · L), where `W` is the total length of the words, `n` the
  stream length and `L` the longest word. In practice walks end early.
- Space: O(W) for the trie.

## Pitfalls

- Inserting words forwards and walking the stream forwards only finds words
  that start at a fixed place; suffixes require the reversed orientation.
- Stop at the **first** marked node: a shorter word that matches is enough.
- Don't rebuild the received text as a new string each step; index into it.
- Aho–Corasick also solves this in O(W + n), at the cost of computing failure
  links.
