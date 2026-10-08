A researcher has published `n` papers, and `citations[i]` is the number of
times paper `i` has been cited. Their **h-index** is the largest integer `h`
such that at least `h` of the papers have been cited at least `h` times each.

Return the researcher's h-index. (It is always between `0` and `n`.)

## Example 1

```
citations = [4, 0, 7, 2, 5]
output    = 3    # papers with 4, 7, 5 citations: 3 papers with >= 3 each
```

## Example 2

```
citations = [1, 9, 1]
output    = 1    # only one paper has 2 or more citations
```

## Constraints

- `1 <= len(citations) <= 5000`
- `0 <= citations[i] <= 1000`
