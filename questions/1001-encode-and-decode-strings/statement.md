You need to send a list of strings over a channel that only carries **one**
string. Write two functions:

- `encode(strs)` packs the list into a single string;
- `decode(s)` takes a string produced by `encode` and returns the original
  list, unchanged: same strings, same order, same length.

The strings may contain **any** characters: letters, digits, spaces,
punctuation such as `#`, `,` or `:`, newlines, non-ASCII letters, and so on.
Some of them may be empty, and the list itself may be empty. Whatever
separator or marker you pick can therefore also appear inside the data.

`decode` must rebuild the list from its argument alone: no global variables
or other state shared with `encode`.

## How the tests call your code

Below your two functions the starter code has a small driver,
`encode_and_decode(strs)`, that dojo calls. It runs `encode` on `strs` (and on
another list, to make sure `decode` really reads its input), checks that
`encode` returned a single string, and returns `decode` of that string. The
test passes when the result equals `strs`. If `encode` returns something that
is not a string, the driver returns `["<encode must return one string>"]`,
which never matches.

## Example 1

```
strs   = ["dojo", "keeps", "you", "sharp"]
output = ["dojo", "keeps", "you", "sharp"]
```

One possible encoding is `"4#dojo5#keeps3#you5#sharp"`; any encoding works
as long as `decode` gets the list back.

## Example 2

```
strs   = ["", "3#a", "#", ""]
output = ["", "3#a", "#", ""]
```

Empty strings and strings that look like your own markers must survive the
round trip.

## Constraints

- `0 <= len(strs) <= 1000`
- `0 <= len(strs[i]) <= 200`
- `strs[i]` may contain any Unicode characters.
