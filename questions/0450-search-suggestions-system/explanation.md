# Approach: sort, then binary-search each prefix

After sorting, all names that start with a prefix `p` form a contiguous block,
and that block begins at the first name `>= p` (any name starting with `p` is
`>= p`, and any name `>= p` that does not start with `p` is greater than all
of them). So for every typed prefix:

1. Binary-search the first index `start` with `ordered[start] >= prefix`.
2. Take up to three names from `start`, stopping at the first one that does
   not start with `prefix`.

A longer prefix is `>=` the shorter one, so `start` never moves left. Each
search can begin where the previous one ended.

```python
from bisect import bisect_left

def suggested_products(products, search_word):
    ordered = sorted(products)
    result = []
    start = 0
    for k in range(1, len(search_word) + 1):
        prefix = search_word[:k]
        start = bisect_left(ordered, prefix, start)
        suggestions = []
        for word in ordered[start:start + 3]:
            if not word.startswith(prefix):
                break
            suggestions.append(word)
        result.append(suggestions)
    return result
```

**Trie alternative:** insert the sorted products into a trie and store, at
each node, the first three names that pass through it. Then follow
`search_word` down the trie, outputting each node's list (and `[]` once the
path falls off the trie). It uses more memory but answers each character in
O(1) after the build.

## Complexity

- Time: O(S log n + m · (log n + m)), where `S` is the total length of the
  products, `n = len(products)` and `m = len(search_word)`. Sorting dominates
  for typical inputs. The `m` term covers building each prefix and comparing
  it.
- Space: O(n) for the sorted copy, plus the output.

## Pitfalls

- Suggestions must be the lexicographically smallest matches, not the first
  three in the input order.
- Once a prefix matches nothing, every longer prefix also matches nothing, but
  you still need one `[]` entry per remaining character.
- Do not assume three matches exist: stop at the end of the list or at the
  first non-matching name.
- Lexicographic order puts `"lamp"` before `"lamps"`: a name that is a prefix
  of another sorts first.
