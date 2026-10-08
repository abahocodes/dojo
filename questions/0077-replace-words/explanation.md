# Approach: trie of roots

Insert every root into a trie and mark the node where it ends. To replace a
word, walk it down the trie letter by letter. The first marked node reached is
the shortest root that prefixes the word, so we can stop right there. If the
next letter has no child, no root prefixes the word and it stays as it is.

```python
def replace_words(roots, sentence):
    trie = {}
    for root in roots:
        node = trie
        for ch in root:
            node = node.setdefault(ch, {})
        node["$"] = True

    def shortest_root(word):
        node = trie
        for i, ch in enumerate(word):
            node = node.get(ch)
            if node is None:
                return word
            if "$" in node:
                return word[: i + 1]
        return word

    return " ".join(shortest_root(word) for word in sentence.split(" "))
```

**Alternative:** put the roots in a hash set and, for each word, test its
prefixes from shortest to longest. That is O(len(word)²) per word because each
prefix is a new string to hash, while the trie walk is linear.

## Complexity

- Time: O(R + S), where `R` is the total length of the roots and `S` the length
  of the sentence. Each word's walk stops after at most `len(word)` steps.
- Space: O(R) for the trie, plus O(S) for the output.

## Pitfalls

- Taking the first matching root in list order instead of the **shortest**
  one: with roots `["abc", "a"]`, the word `"abcd"` becomes `"a"`.
- Marking only leaves as root ends. A root can be a prefix of another root
  (`"a"` and `"ab"`), so the end marker must live on inner nodes too.
- A word shorter than every root it partially matches (like `"ru"` against
  `"run"`) must stay unchanged.
