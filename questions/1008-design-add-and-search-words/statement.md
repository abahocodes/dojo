Design a word store that supports adding words and looking them up with a
simple pattern language, where a `.` stands for any single letter.

Implement the class `WordDictionary`:

- `WordDictionary()` creates an empty store.
- `addWord(word)` stores `word` (lowercase letters only). Adding a stored
  word again changes nothing.
- `search(word)` returns `true` if some stored word **matches** `word`, and
  `false` otherwise. `word` may contain `.` characters; each one matches
  exactly one letter, any letter. Every other character must match itself.
  A match must cover the whole stored word: `"ca."` matches `"cat"` but not
  `"cats"` or `"ca"`.

## How the tests call your class

Each test is a sequence of operations on one fresh `WordDictionary`.
`ops[i]` names the method (`"addWord"` or `"search"`) and `words[i]` is its
argument. The driver at the bottom of the starter code runs them in order and
collects the return value of every `search` call; `addWord` returns nothing
and adds nothing to the output. You only need to fill in the class.

## Example 1

```
ops    = ["addWord", "addWord", "search", "search", "search", "search"]
words  = ["moon", "mood", "moo.", "m..n", "moo", ".ood"]
output = [true, true, false, true]
```

`"moo."` matches both stored words and `"m..n"` matches `"moon"`. `"moo"` is
too short to match anything, and `".ood"` matches `"mood"`.

## Example 2

```
ops    = ["search", "addWord", "search", "search", "search"]
words  = [".", "x", ".", "..", "y"]
output = [false, true, false, false]
```

## Constraints

- `1 <= len(ops) == len(words) <= 10^4`
- `1 <= len(words[i]) <= 25`
- In `addWord`, `words[i]` consists of lowercase English letters.
- In `search`, `words[i]` consists of lowercase English letters and `.`,
  with at most `2` dots.
