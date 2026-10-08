You are given two strings `s` and `t` of the same length and an integer
`max_cost`. Changing the character `s[i]` into `t[i]` costs
`|code(s[i]) - code(t[i])|`, the absolute difference of their character codes.

Find the longest substring `s[l..r]` that can be turned into the matching
substring `t[l..r]` for a total cost of **at most** `max_cost`, and return its
length. Return `0` if no character can be changed within the budget.

## Example 1

```
s        = "abcd"
t        = "bcdf"
max_cost = 3
output   = 3    # costs are 1, 1, 1, 2; "abc" -> "bcd" costs 3
```

## Example 2

```
s        = "hello"
t        = "hallo"
max_cost = 0
output   = 3    # "llo" already matches, so it costs 0
```

## Constraints

- `1 <= len(s) == len(t) <= 10^5`
- `0 <= max_cost <= 10^6`
- `s` and `t` consist of lowercase English letters.
