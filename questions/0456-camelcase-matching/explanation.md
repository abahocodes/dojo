# Approach: greedy two pointers per query

A query matches when the pattern is a subsequence of it and every unmatched
query character is lowercase. Scan the query, keeping `j`, the number of
pattern characters matched so far:

- if the character equals `pattern[j]`, consume it (`j += 1`);
- else if it is uppercase, it would have to be inserted, which is forbidden;
- else it is an inserted lowercase letter, fine.

```python
def camel_match(queries, pattern):
    def matches(query):
        j = 0
        for c in query:
            if j < len(pattern) and c == pattern[j]:
                j += 1
            elif "A" <= c <= "Z":
                return False
        return j == len(pattern)

    return [matches(q) for q in queries]
```

Matching greedily is safe: when a lowercase query character equals
`pattern[j]`, using it now never hurts, because anything a later copy could
match, this one can too. When an uppercase character equals `pattern[j]`, it
must be matched since it cannot be skipped.

The trie tag: if many queries had to be tested against many patterns, the
queries could share prefixes in a trie, but for one pattern the linear scan is
optimal.

## Complexity

- Time: O(total length of queries + len(queries) * len(pattern)) worst case,
  O(sum of query lengths) in practice.
- Space: O(1) besides the output.

## Pitfalls

- Forgetting to check `j == len(pattern)` at the end: `"abc"` does not match
  `"abcd"`.
- Treating extra uppercase letters at the end as fine: `"GUNX"` fails `"GUN"`.
- Pattern letters may be lowercase too; they still have to appear in order.
