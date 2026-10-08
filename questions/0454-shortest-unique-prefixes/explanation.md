# Approach: trie with pass-through counts

Build a trie of all words, storing at each node the number of words whose path
goes through it. A prefix belongs to only one word exactly when its node's
count is 1. For each word, walk its path and cut at the first node with count 1.

```python
def shortest_unique_prefixes(words):
    children = [{}]
    count = [0]
    for w in words:
        node = 0
        for c in w:
            nxt = children[node].get(c)
            if nxt is None:
                nxt = len(children)
                children[node][c] = nxt
                children.append({})
                count.append(0)
            node = nxt
            count[node] += 1
    result = []
    for w in words:
        node = 0
        length = len(w)
        for i, c in enumerate(w):
            node = children[node][c]
            if count[node] == 1:
                length = i + 1
                break
        result.append(w[:length])
    return result
```

Since no word is a prefix of another, the last node of every word has count 1,
so the walk always stops.

An alternative: sort the words. The words that share the longest prefix with
`w` are its neighbours in sorted order, so `w` needs one character more than
its longest common prefix with either neighbour.

## Complexity

- Time: O(T), where T is the total number of characters (two passes).
- Space: O(T) trie nodes.

## Pitfalls

- Testing each candidate prefix against every other word: O(n * T) overall.
- Cutting at the first node with a single child. A node with one child can
  still be shared by several words that branch apart further down; only the
  pass-through count says how many words use it.
- Returning the prefixes in sorted order instead of input order.
