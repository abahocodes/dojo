# Approach: length-prefixed chunks

Write every string as `<length>#<string>` and concatenate the chunks. The
length tells `decode` exactly where each string ends, so the content itself
is never scanned for separators: `#`, digits or anything else inside a string
is just data.

```python
def encode(strs):
    return "".join(f"{len(s)}#{s}" for s in strs)

def decode(s):
    result = []
    i = 0
    while i < len(s):
        j = s.index("#", i)          # end of the length field
        length = int(s[i:j])
        result.append(s[j + 1 : j + 1 + length])
        i = j + 1 + length
    return result
```

`decode` looks for `#` only where a length field starts. The digits before it
were written by `encode`, so the first `#` after `i` is always the marker.

An empty list encodes to `""`, which decodes to `[]`. An empty string encodes
to `"0#"`, which decodes to `[""]`, so the two stay distinct.

Lengths are counted in whatever unit the language uses for string indexing
(code points in Python, UTF-16 units in JavaScript and Java, bytes in C++ and
Go). That is fine as long as `encode` and `decode` agree.

## Complexity

- Time: O(n) for `n` total characters, for both functions.
- Space: O(n) for the encoded string and the decoded list.

## Pitfalls

- A plain separator (`","`, `"|"`, even a rare character) fails when a string
  contains it. Escaping fixes that, but it is easy to get wrong.
- Searching for `#` from the current position *inside* the data, rather than
  jumping past the string using its length, mistakes content for markers.
- Lengths can have several digits (`"12#..."`); read up to the marker, not one
  character.
- Make sure `[]` and `[""]` encode differently.
