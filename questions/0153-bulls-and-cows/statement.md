In a code-breaking game one player picks a `secret` string of digits and
the other answers with a `guess` of the same length. The reply to a guess
has two numbers:

- **Bulls:** positions where `guess` has the same digit as `secret`.
- **Cows:** digits of `guess` that do appear in `secret`, but not at that
  position. Bull positions are set aside on both sides first, and each
  remaining digit of `secret` can be matched to at most one remaining digit
  of `guess`. So for each digit `d`, cows gain
  `min(remaining count of d in secret, remaining count of d in guess)`.

Return the reply as the string `"xAyB"`, where `x` is the number of bulls
and `y` the number of cows (for example `"1A3B"`).

## Example 1

```
secret = "2718"
guess  = "7210"
output = "1A2B"   # bull: '1' at index 2; cows: '7' and '2'
```

## Example 2

```
secret = "4455"
guess  = "5444"
output = "1A2B"   # bull: '4' at index 1; cows: one '4' and one '5'
```

## Constraints

- `1 <= len(secret) == len(guess) <= 1000`
- Both strings contain only the digits `0`-`9`.
