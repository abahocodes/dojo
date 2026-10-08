You are given two lists of lowercase words, `words_container` and
`words_query`. For each query, pick the container word that shares the
**longest common suffix** with it. Break ties as follows:

1. prefer the container word with the smallest length;
2. if still tied, prefer the smallest index.

If a query shares no non-empty suffix with any container word, every word
shares the empty suffix with it, so the same tie-break picks the shortest
container word (smallest index among equals).

Return the chosen container indices (0-based), one per query, in query order.

## Example 1

```
words_container = ["walking", "talking", "king", "ring"]
words_query     = ["seeking", "bring", "abc"]
output          = [2, 3, 2]
```

`"seeking"` shares `"king"` with indices 0, 1 and 2; `"king"` is the
shortest. `"bring"` shares `"ring"` only with index 3. `"abc"` shares nothing,
so the shortest words `"king"` and `"ring"` tie and index 2 wins.

## Example 2

```
words_container = ["ab", "b"]
words_query     = ["cab", "x"]
output          = [0, 1]
```

## Constraints

- `1 <= len(words_container), len(words_query) <= 10^4`
- `1 <= len(word) <= 5000` for every word
- The total length of each list is at most `5 * 10^5`.
- Words consist of lowercase English letters.
