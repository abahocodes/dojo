# Approach: walk a trie from every start index

Insert all the words into a trie and mark each node where a word ends. For
every start index `i`, follow the trie along `text[i], text[i + 1], ...`.
Whenever the current node is marked, `text[i..j]` is a word, so record
`[i, j]`. As soon as a character has no matching child, no word can begin at
`i` with this longer slice, so move to the next start.

Looping `i` upward and `j` upward inside produces the pairs already in the
required order, so no sort is needed.

```python
def index_pairs(text, words):
    children = [{}]
    ends = [False]
    for word in words:
        node = 0
        for ch in word:
            nxt = children[node].get(ch)
            if nxt is None:
                nxt = len(children)
                children[node][ch] = nxt
                children.append({})
                ends.append(False)
            node = nxt
        ends[node] = True

    result = []
    for i in range(len(text)):
        node = 0
        for j in range(i, len(text)):
            node = children[node].get(text[j])
            if node is None:
                break
            if ends[node]:
                result.append([i, j])
    return result
```

**Simpler alternative:** put the words in a set and test every slice
`text[i:j + 1]` with `j - i < 50`. Each test hashes a fresh substring, so it
costs O(T · L²) instead of O(T · L).

## Complexity

- Time: O(W + T · L), where `W` is the total length of the words, `T` is
  `len(text)` and `L` is the longest word length.
- Space: O(W) for the trie, plus the output.

## Pitfalls

- `j` is inclusive: the pair for `text[3:6]` in Python slicing is `[3, 5]`.
- Using `str.find` once per word finds only the first occurrence. Every
  occurrence, including overlapping ones, must be reported.
- Collecting matches word by word and forgetting to sort them by `(i, j)`.
- Do not stop at the first marked node: a longer word may share the prefix
  (`"a"` and `"aa"`).
