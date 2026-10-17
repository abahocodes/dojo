# Approach: trie plus depth-first search for wildcards

Store the words in a trie: each node is a prefix, children are keyed by
letter, and a flag marks nodes where an added word ends.

`search` walks the pattern alongside the trie. A letter forces one child; a
`.` could be any letter, so the search branches into every existing child
and succeeds if any branch does. At the end of the pattern, the node must be
a word end, so a pattern never matches a longer or shorter word.

```python
class WordDictionary:
    def __init__(self):
        self.root = {}            # letter -> child dict; "$" marks a word end

    def addWord(self, word):
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word):
        def match(node, i):
            if i == len(word):
                return "$" in node
            ch = word[i]
            if ch != ".":
                child = node.get(ch)
                return child is not None and match(child, i + 1)
            return any(k != "$" and match(child, i + 1)
                       for k, child in node.items())
        return match(self.root, 0)
```

## Complexity

- `addWord`: O(L) for a word of length `L`.
- `search`: O(L) without dots. Each dot multiplies the branching by at most
  26, so with at most two dots a search visits O(26^2 · L) nodes in the
  worst case, and usually far fewer because the trie is sparse.
- Space: O(total length of added words).

## Pitfalls

- Returning `true` when the pattern runs out on any node instead of only on
  a word-end node: `"moo"` must not match `"moon"`.
- Treating `.` as "zero or more letters" (that's regex `.*`). It is exactly
  one letter.
- Keeping all words in a list and comparing each one is O(n · L) per search,
  too slow when there are thousands of words and searches.
- Without a trie, grouping words by length and comparing within the group
  helps but still degrades when many words share a length.
