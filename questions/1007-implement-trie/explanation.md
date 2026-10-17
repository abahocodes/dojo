# Approach: tree of letters with end-of-word flags

Every node represents the prefix spelled by the path from the root to it.
A node keeps its children by letter and a flag saying whether some inserted
word ends exactly there.

- `insert` walks the word letter by letter, creating children that don't
  exist yet, and marks the last node as a word end.
- Both queries share one helper that walks a string and returns the node it
  reaches, or nothing if a letter has no child.
- `startsWith` only needs the walk to succeed; `search` also needs the end
  flag, because a prefix of a stored word is not itself stored.

```python
class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word):
        node = self.root
        for ch in word:
            node = node.children.setdefault(ch, TrieNode())
        node.is_end = True

    def _walk(self, s):
        node = self.root
        for ch in s:
            node = node.children.get(ch)
            if node is None:
                return None
        return node

    def search(self, word):
        node = self._walk(word)
        return node is not None and node.is_end

    def startsWith(self, prefix):
        return self._walk(prefix) is not None
```

In Java, C++ and Go an array of 26 child pointers indexed by `ch - 'a'` is
the usual choice: it is faster than a hash map and the alphabet is small.

## Complexity

- Time: O(L) per operation, where `L` is the length of its argument.
- Space: O(total length of inserted words) nodes in the worst case (no
  shared prefixes), each with up to 26 child slots.

## Pitfalls

- Answering `search` with "the walk succeeded" returns `true` for prefixes
  like `"plan"` when only `"planet"` is stored. You need the end flag.
- Setting the end flag on every node during `insert` instead of only the
  last one.
- Keeping a set of all prefixes works but uses O(L^2) memory per word, which
  blows up on long words.
