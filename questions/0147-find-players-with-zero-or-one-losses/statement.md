You are given the results of some matches as a list `matches`, where
`matches[i] = [winner, loser]` means player `winner` beat player `loser` in
match `i`. There are no draws.

Return a list `[a, b]` where

- `a` lists every player who played at least one match and never lost, and
- `b` lists every player who lost exactly one match.

Both lists must be in increasing order. Players who never played appear in
neither list.

## Example 1

```
matches = [[2, 5], [1, 5], [3, 2], [1, 4], [6, 7], [7, 3]]
output  = [[1, 6], [2, 3, 4, 7]]   # 5 lost twice
```

## Example 2

```
matches = [[4, 1], [1, 4]]
output  = [[], [1, 4]]
```

## Constraints

- `1 <= len(matches) <= 10^5`
- `matches[i].length == 2`
- `1 <= winner, loser <= 10^5`
- `winner != loser`
