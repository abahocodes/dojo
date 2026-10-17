Build a **trie** (prefix tree): a structure that stores lowercase words and
answers two kinds of question quickly — is this exact word stored, and is any
stored word starting with this prefix?

Implement the class `Trie`:

- `Trie()` creates an empty trie.
- `insert(word)` stores `word`. Inserting a word that is already stored
  changes nothing.
- `search(word)` returns `true` if `word` itself was inserted earlier, and
  `false` otherwise. Being the prefix of a stored word is not enough.
- `startsWith(prefix)` returns `true` if at least one stored word begins with
  `prefix` (a stored word counts as its own prefix), and `false` otherwise.

## How the tests call your class

Each test is a sequence of operations on one fresh `Trie`. `ops[i]` names the
method (`"insert"`, `"search"` or `"startsWith"`) and `words[i]` is its
argument. The driver at the bottom of the starter code runs them in order and
collects the return value of every `search` and `startsWith` call; `insert`
returns nothing and adds nothing to the output. You only need to fill in the
class.

## Example 1

```
ops    = ["insert", "search", "search", "startsWith", "insert", "search"]
words  = ["planet", "planet", "plan", "plan", "plan", "plan"]
output = [true, false, true, true]
```

`"plan"` is not stored at first, only a prefix of `"planet"`, so `search`
says `false` while `startsWith` says `true`. After inserting it, `search`
finds it.

## Example 2

```
ops    = ["search", "insert", "startsWith", "startsWith", "search"]
words  = ["a", "bee", "be", "bed", "bee"]
output = [false, true, false, true]
```

## Constraints

- `1 <= len(ops) == len(words) <= 3 * 10^4`
- `1 <= len(words[i]) <= 2000`
- `words[i]` consists of lowercase English letters.
- The total length of all arguments is at most `10^5`.
