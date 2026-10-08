# Approach: trie-guided backtracking

Build a trie from `words`, storing each complete word on the node where it
ends. Then start a DFS from every cell. The DFS carries the trie node for the
letters spelled so far; stepping to a neighbour is only worthwhile if the
neighbour's letter is a child of that node. That single check prunes every
path that isn't a prefix of some word, and every word sharing a prefix is
searched for at the same time.

When a node holding a word is reached, the word is recorded and removed from
the node so it is reported once. After exploring a node, if it has no children
and no word left, it is deleted from its parent: later searches no longer walk
into branches whose words have all been found.

```python
def find_words(board, words):
    root = {}
    for word in words:
        node = root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = word

    rows, cols = len(board), len(board[0])
    found = []

    def dfs(r, c, parent):
        ch = board[r][c]
        node = parent.get(ch)
        if node is None:
            return
        word = node.pop("$", None)
        if word is not None:
            found.append(word)
        board[r][c] = "#"                      # on the current path
        for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
            if 0 <= nr < rows and 0 <= nc < cols and board[nr][nc] != "#":
                dfs(nr, nc, node)
        board[r][c] = ch
        if not node:
            del parent[ch]                     # nothing left to find here

    for r in range(rows):
        for c in range(cols):
            dfs(r, c, root)
    return found
```

The recursion depth is at most the length of the longest word (10), so
recursion is safe here.

## Complexity

- Time: O(rows × cols × 4 × 3^(L−1)) in the worst case, where `L` is the
  longest word length: from each cell the path branches into at most 3 new
  directions per step. Building the trie is O(total letters in `words`). In
  practice the trie and the pruning cut the search far below this bound.
- Space: O(total letters in `words`) for the trie, plus O(L) recursion.

## Pitfalls

- Running a separate Word Search for every word is correct but much slower
  when there are many words with shared prefixes.
- Forgetting to restore the cell after the DFS returns corrupts later
  searches.
- Without clearing the word after finding it, the same word is reported again
  from a different starting cell or path.
- A word can't reuse a cell: on a one-cell board `"a"`, the word `"aa"` is not
  present.
