# Approach: reversed trie with the best index stored at each node

Insert each container word, reversed, into a trie. A node corresponds to a
suffix, and the words passing through it are exactly the container words that
end with that suffix. Store at every node the preferred word among them:
shortest length, then smallest index. Because words are inserted in index
order, a later word replaces the stored one only if it is strictly shorter.
The root covers the empty suffix, so it stores the best word overall.

For a query, walk its reversed characters down the trie as far as possible.
The node where the walk stops is the longest shared suffix, and its stored
index is the answer.

```python
def string_indices(words_container, words_query):
    children = [{}]
    best = [0]
    for i, w in enumerate(words_container):
        if len(w) < len(words_container[best[0]]):
            best[0] = i
        node = 0
        for c in reversed(w):
            nxt = children[node].get(c)
            if nxt is None:
                nxt = len(children)
                children[node][c] = nxt
                children.append({})
                best.append(i)
            elif len(w) < len(words_container[best[nxt]]):
                best[nxt] = i
            node = nxt
    result = []
    for q in words_query:
        node = 0
        for c in reversed(q):
            nxt = children[node].get(c)
            if nxt is None:
                break
            node = nxt
        result.append(best[node])
    return result
```

The Java, C++, Go and JavaScript solutions store the trie as one flat array
of 26 slots per node, sized by the total container length.

## Complexity

- Time: O(C + Q), the total lengths of the container and query lists.
- Space: O(C) trie nodes (26 slots each in the array form).

## Pitfalls

- Comparing every query with every container word: up to 10^8 pairs.
- Forgetting the root: a query with no shared suffix still gets an answer,
  the shortest container word.
- Updating the stored index on equal length, which breaks the
  smallest-index tie-break.
- Walking queries forward instead of reversed.
