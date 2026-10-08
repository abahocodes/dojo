You are given a list of `queries` and a `pattern`. A query **matches** the
pattern if you can turn the pattern into exactly the query by inserting
**lowercase** letters, any number of them, at any positions (including the
start and the end). Inserting uppercase letters is not allowed, and the
pattern's own letters must stay in order.

Return a list of booleans, one per query in input order, telling whether that
query matches.

## Example 1

```
queries = ["GetUserName", "GetUsername", "GoUpNow", "GUN", "GetUserNameX"]
pattern = "GUN"
output  = [true, false, true, true, false]
```

`"GetUsername"` has no uppercase `N`, and `"GetUserNameX"` would need an
inserted uppercase `X`.

## Example 2

```
queries = ["abc", "aXbc", "axc"]
pattern = "ac"
output  = [true, false, true]
```

## Constraints

- `1 <= len(queries) <= 100`
- `1 <= len(queries[i]), len(pattern) <= 100`
- All strings consist of English letters (both cases).
