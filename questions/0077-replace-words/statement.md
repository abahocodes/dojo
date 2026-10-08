You are given a list of word `roots` and a `sentence` made of lowercase words
separated by single spaces. Rewrite the sentence by replacing every word that
**starts with** at least one root with the **shortest** such root. Words that
don't start with any root stay as they are. A word that is exactly equal to a
root counts as starting with it.

Return the rewritten sentence, with words still separated by single spaces.

## Example 1

```
roots    = ["sun", "moon", "star"]
sentence = "sunflowers bloom under moonlight and starry skies"
output   = "sun bloom under moon and star skies"
```

## Example 2

```
roots    = ["a", "ab", "abc"]
sentence = "abcd bcd abd"
output   = "a bcd a"     # "a" is the shortest root that "abcd" starts with
```

## Constraints

- `1 <= len(roots) <= 1000`
- `1 <= len(roots[i]) <= 100`
- `1 <= len(sentence) <= 20000`
- Roots and sentence words contain only lowercase English letters; roots may
  repeat.
- `sentence` has no leading or trailing spaces, and words are separated by
  exactly one space.
