You have a pile of stones; `stones[i]` is the weight of the `i`-th stone.

Play the following game until at most one stone is left. Each turn, pick the
**two heaviest** stones, with weights `x <= y`, and smash them together:

- if `x == y`, both stones are destroyed;
- otherwise the lighter stone is destroyed and the heavier one now weighs
  `y - x` (it goes back into the pile).

Return the weight of the stone that remains at the end, or `0` if no stones
remain.

## Example 1

```
stones = [3, 9, 4, 1, 6]
output = 1
```

Smash 9 and 6 → 3, pile `[3, 4, 1, 3]`. Smash 4 and 3 → 1, pile `[1, 3, 1]`.
Smash 3 and 1 → 2, pile `[1, 2]`. Smash 2 and 1 → 1, pile `[1]`.

## Example 2

```
stones = [5, 5]
output = 0
```

## Constraints

- `1 <= len(stones) <= 10^4`
- `1 <= stones[i] <= 1000`
