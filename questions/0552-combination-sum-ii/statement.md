You have a pile of number cards, given as the list `candidates` of positive
integers. Several cards may show the same number. Find every way to pick
some of the cards so that their numbers add up to exactly `target`. Each
card can be picked **at most once**.

Two picks are the same if they contain the same numbers the same number of
times, regardless of which physical cards were used, so with two `1` cards,
taking either one gives the same combination `[1, 5]`. Return each distinct
combination once, in any order, with its values in any order. If no
combination works, return an empty list.

## Example 1

```
candidates = [4, 1, 3, 1, 2, 5]
target     = 6
output     = [[1, 1, 4], [1, 2, 3], [1, 5], [2, 4]]
```

`[1, 5]` can be made with either `1` card but is listed once. `[3, 3]` is
not allowed because there is only one `3` card.

## Example 2

```
candidates = [3, 3, 5]
target     = 4
output     = []
```

## Example 3

```
candidates = [2, 2, 2, 2]
target     = 6
output     = [[2, 2, 2]]
```

## Constraints

- `1 <= len(candidates) <= 100`
- `1 <= candidates[i] <= 50`
- `1 <= target <= 30`
