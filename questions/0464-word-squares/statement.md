You are given a list of **distinct** lowercase words, all of the same length
`L`.

A **word square** is a sequence of `L` words, taken from the list (the same
word may be used more than once), arranged as the rows of an `L × L` grid so
that for every `k`, the `k`-th row reads exactly the same as the `k`-th column.

Return every word square that can be built. Each square is a list of its `L`
words in row order (top to bottom). The squares themselves may be returned in
any order.

## Example 1

```
words  = ["tea", "ear", "arc", "ink", "oak"]
output = [["tea", "ear", "arc"]]
```

```
t e a
e a r
a r c
```

Row 1 `tea` matches column 1 `t-e-a`, row 2 `ear` matches column 2 `e-a-r`,
and row 3 `arc` matches column 3 `a-r-c`.

## Example 2

```
words  = ["ab", "ba"]
output = [["ab", "ba"], ["ba", "ab"]]
```

## Constraints

- `1 <= len(words) <= 1000`
- `1 <= L <= 4`, and every word has length `L`
- All words are distinct and consist of lowercase English letters.
