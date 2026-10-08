# Approach: split, then check a bijection with two maps

After splitting `s` on spaces, the question is the same as for isomorphic
strings, with words in place of characters: the pairing between letters and
words must be consistent in **both** directions. Keep `letter_to_word` and
`word_to_letter`; every pair `(c, w)` must agree with whatever either map
already says.

```python
def word_pattern(pattern, s):
    words = s.split(' ')
    if len(words) != len(pattern):
        return False
    letter_to_word = {}
    word_to_letter = {}
    for c, w in zip(pattern, words):
        if letter_to_word.setdefault(c, w) != w or word_to_letter.setdefault(w, c) != c:
            return False
    return True
```

## Complexity

- Time: O(n + m) for a pattern of length n and a string of length m; each word
  is hashed a constant number of times.
- Space: O(m) for the words and the maps.

## Pitfalls

- Forgetting the length check. `zip` stops at the shorter sequence, so
  `"aaa"` / `"hi hi"` would wrongly pass.
- Checking only letter -> word, which accepts `"ab"` / `"same same"`.
- Using one shared map for both directions: a letter `"a"` and a word `"a"`
  collide (`"ab"` / `"b a"` is `true`).
