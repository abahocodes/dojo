You are given a string `order` of **distinct** lowercase letters, which
defines a ranking of those letters, and a string `s` of lowercase letters.

Rearrange the characters of `s` as follows:

- first, all letters of `s` that appear in `order`, arranged by their
  position in `order` (all copies of a letter together);
- then, all letters of `s` that do not appear in `order`, in the same
  relative order as they appear in `s`.

Return the rearranged string.

## Example 1

```
order  = "tea"
s      = "eagerest"
output = "teeeagrs"   # t, then e x3, then a, then g r s as they appear in s
```

## Example 2

```
order  = "zyx"
s      = "xmaxyzby"
output = "zyyxxmab"   # m, a, b are unranked and keep their order from s
```

## Constraints

- `1 <= len(order) <= 26`
- `1 <= len(s) <= 200`
- `order` has no repeated letters; both strings contain only lowercase
  English letters.
