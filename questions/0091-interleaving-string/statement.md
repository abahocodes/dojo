Two people dictate words letter by letter, `s1` and `s2`, and a stenographer
writes down the letters as they arrive, producing `s3`. Neither speaker's
letters ever get reordered, but the stenographer may switch between the two
speakers at any point.

Return `true` if `s3` can be formed by **interleaving** `s1` and `s2`: every
letter of `s1` and of `s2` is used exactly once, each string's letters appear
in `s3` in their original order, and nothing else is in `s3`.

## Example 1

```
s1     = "dog"
s2     = "cat"
s3     = "dcoagt"
output = true     # d(s1) c(s2) o(s1) a(s2) g(s1) t(s2)
```

## Example 2

```
s1     = "ab"
s2     = "ab"
s3     = "abba"
output = false
```

## Constraints

- `0 <= len(s1), len(s2) <= 100`
- `0 <= len(s3) <= 200`
- All strings contain only lowercase English letters.
