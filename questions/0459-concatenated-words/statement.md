You are given a list of distinct lowercase `words`. A word is
**concatenated** if it can be written as two or more shorter words from the
same list placed back to back. The same word may be used more than once.

Return every concatenated word in the list. The order of the result does not
matter.

## Example 1

```
words  = ["sun", "flower", "sunflower", "sunsun", "moon", "moonsunflower"]
output = ["sunflower", "sunsun", "moonsunflower"]
```

`"moonsunflower"` is `"moon" + "sun" + "flower"` (or `"moon" + "sunflower"`).

## Example 2

```
words  = ["a", "b", "ab", "abc"]
output = ["ab"]
```

`"abc"` would need a word `"c"` or `"bc"`, which is not in the list.

## Constraints

- `1 <= len(words) <= 10^4`
- `1 <= len(words[i]) <= 30`
- The total number of characters is at most `10^5`.
- Words are distinct and consist of lowercase English letters.
