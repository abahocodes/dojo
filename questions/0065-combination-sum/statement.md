You are given a list `candidates` of **distinct** positive integers and a
positive integer `target`. Find every combination of candidates whose values
add up to exactly `target`. Any candidate may be used as many times as you
like.

Two combinations are the same if they use the same values the same number of
times, regardless of order, so `[2, 3, 3]` and `[3, 2, 3]` count once. Return
each distinct combination once, in any order, with its values in any order.
If no combination works, return an empty list.

## Example 1

```
candidates = [3, 5, 2]
target     = 8
output     = [[3, 5], [2, 3, 3], [2, 2, 2, 2]]
```

## Example 2

```
candidates = [4]
target     = 3
output     = []
```

## Constraints

- `1 <= len(candidates) <= 30`
- `2 <= candidates[i] <= 40`
- All values in `candidates` are distinct.
- `1 <= target <= 30`
