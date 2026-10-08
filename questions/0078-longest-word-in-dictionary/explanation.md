# Approach: trie traversal through word ends

Insert every word into a trie and store the word on the node where it ends.
Walking from the root, a node is reachable through word ends only if every
prefix along the way is itself a word — which is exactly the buildable
condition. So traverse the trie, stepping only into children that hold a
word, and keep the best word seen: longer wins, and among equal lengths the
alphabetically smaller one wins. An explicit stack keeps the traversal
iterative.

```python
def longest_word(words):
    trie = {}
    for word in words:
        node = trie
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = word

    best = ""
    stack = [trie]
    while stack:
        node = stack.pop()
        for ch, child in node.items():
            if ch == "$" or "$" not in child:
                continue
            word = child["$"]
            if len(word) > len(best) or (len(word) == len(best) and word < best):
                best = word
            stack.append(child)
    return best
```

**Alternative:** sort the words, then scan them with a set of buildable words:
a word is buildable if it has length 1 or its prefix without the last letter
is already in the set. Sorting puts every prefix before the words that extend
it. This is O(W log W · L) because of string comparisons in the sort.

## Complexity

- Time: O(W · L), where `W` is the number of words and `L` the maximum word
  length: each letter is inserted once and each trie node is visited at most
  once.
- Space: O(W · L) for the trie.

## Pitfalls

- A word whose letters are all in the trie is not necessarily buildable: in
  `["ab", "abc"]` there is no `"a"`, so nothing is buildable and the answer
  is `""`.
- The tie-break is alphabetical among the **longest** buildable words, not
  among all of them.
- Duplicate words in the input must not change the answer.
