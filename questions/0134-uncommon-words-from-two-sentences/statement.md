You are given two sentences `s1` and `s2`. Each is a sequence of lowercase
words separated by single spaces, with no leading or trailing spaces.

Call a word **uncommon** if it appears exactly once when both sentences are
taken together: once in one sentence and not at all in the other. Return all
uncommon words, in any order. If there are none, return an empty list.

## Example 1

```
s1     = "red fish blue fish"
s2     = "one fish two"
output = ["red", "blue", "one", "two"]    # any order
```

## Example 2

```
s1     = "go go"
s2     = "stop"
output = ["stop"]    # "go" appears twice in s1
```

## Constraints

- `1 <= len(s1), len(s2) <= 200`
- Both sentences consist of lowercase English letters and single spaces,
  and neither starts or ends with a space.
