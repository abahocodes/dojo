You are given a string `chars`. Shorten it with **run-length encoding**:
scan the string and split it into maximal runs of the same character. Each run
is written as its character, followed by the run's length in decimal **only if
the length is greater than 1**. A run of length 1 is written as the bare
character.

Runs are encoded in the order they appear, and the encodings are concatenated.
Return the resulting string. (The result is returned even when it is not
shorter than the input.)

## Example 1

```
chars  = "zzzkpp"
output = "z3kp2"     # "zzz" -> "z3", "k" -> "k", "pp" -> "p2"
```

## Example 2

```
chars  = "bbbbbbbbbbbbq"
output = "b12q"      # a run of twelve 'b's is written with both digits
```

## Constraints

- `1 <= len(chars) <= 2000`
- `chars` consists of printable ASCII characters (codes 32 to 126) other than
  the digits `0`-`9`. Spaces are ordinary characters.
